//! Loading and validation of a module set directory (docs/MODULE-SPEC.md,
//! "Validation Rules").
//!
//! `load_module_set` walks the top level of a modules directory, parses each
//! `*.json5` module, loads referenced sidecars, and runs every validation
//! rule. Loading continues past per-module errors so the report covers the
//! whole set; modules with errors are excluded from the resulting
//! `ModuleSet`, warnings do not exclude.
//!
//! Many spec rules are already guaranteed by `module_spec::parse` (schema
//! version, count/count_from exclusivity, positive sizes, integer size sets,
//! per-type key legality). Parse failures surface here as errors attributed
//! to the module file, so the full rule list still holds for any loaded set.

use std::collections::{BTreeMap, HashMap};
use std::path::Path;
use std::sync::Arc;

use serde::Serialize;

use crate::module_spec::{parse_module, parse_sidecar, ModuleFile, SidecarItem};

mod validate;

#[cfg(test)]
mod tests;

/// One validation finding, attributed to a file and optionally to a module
/// and field.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Issue {
    /// Path relative to the module set directory.
    pub file: String,
    pub module_id: Option<String>,
    pub field_id: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ValidationReport {
    pub errors: Vec<Issue>,
    pub warnings: Vec<Issue>,
}

impl ValidationReport {
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }
}

/// A module that passed validation, with its sidecars loaded.
#[derive(Debug, Clone)]
pub struct LoadedModule {
    /// Module file name within the set directory, e.g. `battle_armament.json5`.
    pub file: String,
    pub module: ModuleFile,
    /// Items of `entry.labels_file`, when present.
    pub entry_labels: Option<Arc<Vec<SidecarItem>>>,
    /// Dropdown options keyed by the `options_file` path as written in the
    /// module (normalized), shared across modules referencing the same file.
    pub options: BTreeMap<String, Arc<Vec<SidecarItem>>>,
}

impl LoadedModule {
    /// Options for a field's `options_file` path as written in the module.
    pub fn options_for(&self, options_file: &str) -> Option<&Arc<Vec<SidecarItem>>> {
        let normalized = validate::normalize_sidecar_path(options_file).ok()?;
        self.options.get(&normalized)
    }
}

/// All valid modules of a set, in file-name order.
#[derive(Debug, Clone, Default)]
pub struct ModuleSet {
    pub modules: Vec<LoadedModule>,
}

impl ModuleSet {
    pub fn find(&self, module_id: &str) -> Option<&LoadedModule> {
        self.modules.iter().find(|m| m.module.id == module_id)
    }
}

/// Cached sidecar files: parsed items, or `None` when the file failed to
/// load (the error is already in the report).
struct SidecarCache {
    items: HashMap<String, Option<Arc<Vec<SidecarItem>>>>,
}

impl SidecarCache {
    fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    /// Loads and caches `normalized` (a path relative to the set root). File
    /// content warnings (duplicate values, empty labels) are reported once
    /// per file, on first load.
    fn load(
        &mut self,
        root: &Path,
        normalized: &str,
        report: &mut ValidationReport,
    ) -> Option<Arc<Vec<SidecarItem>>> {
        if let Some(cached) = self.items.get(normalized) {
            return cached.clone();
        }

        let loaded = Self::read_and_parse(root, normalized, report);
        if let Some(items) = &loaded {
            validate::report_sidecar_content(normalized, items, report);
        }
        self.items.insert(normalized.to_string(), loaded.clone());
        loaded
    }

    fn read_and_parse(
        root: &Path,
        normalized: &str,
        report: &mut ValidationReport,
    ) -> Option<Arc<Vec<SidecarItem>>> {
        let full = root.join(normalized);
        let text = match std::fs::read_to_string(&full) {
            Ok(text) => text,
            Err(err) => {
                report.errors.push(Issue {
                    file: normalized.to_string(),
                    module_id: None,
                    field_id: None,
                    message: format!("failed to read sidecar: {err}"),
                });
                return None;
            }
        };
        match parse_sidecar(&text) {
            Ok(items) => Some(Arc::new(items)),
            Err(err) => {
                report.errors.push(Issue {
                    file: normalized.to_string(),
                    module_id: None,
                    field_id: None,
                    message: err.to_string(),
                });
                None
            }
        }
    }
}

/// Loads every top-level `*.json5` module in `dir`. The `Err` case covers
/// only directory-level I/O; per-file problems land in the report.
pub fn load_module_set(dir: &Path) -> std::io::Result<(ModuleSet, ValidationReport)> {
    let mut report = ValidationReport::default();
    let mut cache = SidecarCache::new();
    let mut modules: Vec<LoadedModule> = Vec::new();
    // Module id -> file that introduced it, for the uniqueness rule.
    let mut seen_ids: HashMap<String, String> = HashMap::new();

    let mut module_files: Vec<String> = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().is_some_and(|e| e == "json5") {
            module_files.push(entry.file_name().to_string_lossy().to_string());
        }
    }
    module_files.sort();

    for file in module_files {
        let issue = |message: String| Issue {
            file: file.clone(),
            module_id: None,
            field_id: None,
            message,
        };

        let text = match std::fs::read_to_string(dir.join(&file)) {
            Ok(text) => text,
            Err(err) => {
                report
                    .errors
                    .push(issue(format!("failed to read module: {err}")));
                continue;
            }
        };
        let module = match parse_module(&text) {
            Ok(module) => module,
            Err(err) => {
                report.errors.push(issue(err.to_string()));
                continue;
            }
        };

        let errors_before = report.errors.len();

        validate::validate_module(&file, &module, &mut report);

        if let Some(prev_file) = seen_ids.get(&module.id) {
            report.errors.push(Issue {
                file: file.clone(),
                module_id: Some(module.id.clone()),
                field_id: None,
                message: format!("module id '{}' is already used by {prev_file}", module.id),
            });
        } else {
            seen_ids.insert(module.id.clone(), file.clone());
        }

        let (entry_labels, options) =
            load_module_sidecars(dir, &file, &module, &mut cache, &mut report);

        if report.errors.len() == errors_before {
            modules.push(LoadedModule {
                file,
                module,
                entry_labels,
                options,
            });
        }
    }

    Ok((ModuleSet { modules }, report))
}

type LoadedSidecars = (
    Option<Arc<Vec<SidecarItem>>>,
    BTreeMap<String, Arc<Vec<SidecarItem>>>,
);

fn load_module_sidecars(
    root: &Path,
    file: &str,
    module: &ModuleFile,
    cache: &mut SidecarCache,
    report: &mut ValidationReport,
) -> LoadedSidecars {
    let resolve = |raw_path: &str,
                   field_id: Option<&str>,
                   cache: &mut SidecarCache,
                   report: &mut ValidationReport|
     -> Option<(String, Arc<Vec<SidecarItem>>)> {
        match validate::normalize_sidecar_path(raw_path) {
            Ok(normalized) => cache
                .load(root, &normalized, report)
                .map(|items| (normalized, items)),
            Err(reason) => {
                report.errors.push(Issue {
                    file: file.to_string(),
                    module_id: Some(module.id.clone()),
                    field_id: field_id.map(str::to_string),
                    message: format!("invalid sidecar path '{raw_path}': {reason}"),
                });
                None
            }
        }
    };

    let entry_labels = module
        .entry
        .labels_file
        .as_deref()
        .and_then(|path| resolve(path, None, cache, report))
        .map(|(_, items)| items);

    if let Some(labels) = &entry_labels {
        validate::report_entry_count_overflow(file, module, labels, report);
    }

    let mut options = BTreeMap::new();
    for field in &module.fields {
        let Some(options_file) = field.options_file() else {
            continue;
        };
        let Some((normalized, items)) = resolve(options_file, Some(field.id()), cache, report)
        else {
            continue;
        };
        validate::report_option_size_overflow(file, module, field, &normalized, &items, report);
        options.insert(normalized, items);
    }

    (entry_labels, options)
}
