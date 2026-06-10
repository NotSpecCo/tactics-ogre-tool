//! Validation rules over parsed modules and sidecars.
//!
//! Rules already guaranteed by `module_spec::parse` (schema version, entry
//! count exclusivity, positive sizes, integer size sets, per-type field key
//! legality) are not re-checked here; a parse failure is reported as an
//! error by the loader, so the spec's full rule list holds for any loaded
//! module.

use std::collections::{BTreeMap, HashSet};

use globset::{GlobBuilder, GlobSet, GlobSetBuilder};

use crate::module_spec::{CountSpec, Field, ModuleFile, SidecarItem};

use super::{Issue, ValidationReport};

/// Compiles a module's `files` patterns into a matcher with the spec's glob
/// semantics: `*` and `?` never match `/`, and matching is case-insensitive.
pub fn build_matcher(files: &[String]) -> Result<GlobSet, (String, String)> {
    let mut builder = GlobSetBuilder::new();
    for pattern in files {
        let glob = GlobBuilder::new(pattern)
            .literal_separator(true)
            .case_insensitive(true)
            .build()
            .map_err(|err| (pattern.clone(), err.to_string()))?;
        builder.add(glob);
    }
    builder
        .build()
        .map_err(|err| (String::new(), err.to_string()))
}

/// Module validation rule 3: `^[a-z][a-z0-9_]*$`.
fn is_valid_id(id: &str) -> bool {
    let mut chars = id.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    first.is_ascii_lowercase()
        && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

/// Module validation rules 4 and 5 for one `files` entry: a relative POSIX
/// path or glob that cannot resolve outside the game root.
fn check_files_pattern(pattern: &str) -> Result<(), String> {
    if pattern.is_empty() {
        return Err("must not be empty".to_string());
    }
    if pattern.starts_with('/') {
        return Err("must not begin with '/'".to_string());
    }
    if pattern.contains('\\') {
        return Err("must use POSIX '/' separators".to_string());
    }
    if pattern.ends_with('/') {
        return Err("directory targets are invalid".to_string());
    }
    if pattern.split('/').any(|segment| segment == "..") {
        return Err("must not contain '..'".to_string());
    }
    if pattern.split('/').any(|segment| segment.is_empty()) {
        return Err("must not contain empty path segments".to_string());
    }
    Ok(())
}

/// Normalizes a sidecar path relative to the module file's directory and
/// requires it to stay within the module tree (spec "Path Rules").
pub fn normalize_sidecar_path(path: &str) -> Result<String, String> {
    if path.is_empty() {
        return Err("must not be empty".to_string());
    }
    if path.starts_with('/') {
        return Err("must be relative, not absolute".to_string());
    }
    if path.contains('\\') {
        return Err("must use POSIX '/' separators".to_string());
    }

    let mut segments: Vec<&str> = Vec::new();
    for segment in path.split('/') {
        match segment {
            "" => return Err("must not contain empty path segments".to_string()),
            "." => {}
            ".." => {
                if segments.pop().is_none() {
                    return Err("escapes the module tree".to_string());
                }
            }
            other => segments.push(other),
        }
    }
    if segments.is_empty() {
        return Err("resolves to no file".to_string());
    }
    Ok(segments.join("/"))
}

/// Runs the module and field validation rules that the parser does not
/// already guarantee.
pub fn validate_module(file: &str, module: &ModuleFile, report: &mut ValidationReport) {
    let issue = |field_id: Option<&str>, message: String| Issue {
        file: file.to_string(),
        module_id: Some(module.id.clone()),
        field_id: field_id.map(str::to_string),
        message,
    };

    // Module rule 3: id format.
    if !is_valid_id(&module.id) {
        report.errors.push(issue(
            None,
            format!("module id '{}' must match ^[a-z][a-z0-9_]*$", module.id),
        ));
    }

    // Module rules 4 and 5: files list.
    if module.files.is_empty() {
        report
            .errors
            .push(issue(None, "files must not be empty".to_string()));
    }
    for pattern in &module.files {
        if let Err(reason) = check_files_pattern(pattern) {
            report
                .errors
                .push(issue(None, format!("files entry '{pattern}': {reason}")));
        }
    }

    // Module rule 6: base_offset must leave room for the 16-byte block
    // header on header-driven tables.
    if matches!(module.entry.count, CountSpec::Header { .. }) && module.base_offset < 0x10 {
        report.errors.push(issue(
            None,
            format!(
                "base_offset 0x{:02x} must be at least 0x10 when entry.header is true",
                module.base_offset
            ),
        ));
    }

    // Module rule 11: fields non-empty.
    if module.fields.is_empty() {
        report
            .errors
            .push(issue(None, "fields must not be empty".to_string()));
    }

    // Module rules 3 and 12: field id format and uniqueness.
    let mut seen_field_ids: HashSet<&str> = HashSet::new();
    for field in &module.fields {
        if !is_valid_id(field.id()) {
            report.errors.push(issue(
                Some(field.id()),
                format!("field id '{}' must match ^[a-z][a-z0-9_]*$", field.id()),
            ));
        }
        if !seen_field_ids.insert(field.id()) {
            report.errors.push(issue(
                Some(field.id()),
                format!("field id '{}' is not unique within the module", field.id()),
            ));
        }
    }

    // Field rule 5: offset + size <= entry.size.
    for field in &module.fields {
        if let Some((offset, size)) = field.storage() {
            let end = offset as u128 + size as u128;
            if end > module.entry.size as u128 {
                report.errors.push(issue(
                    Some(field.id()),
                    format!(
                        "field storage 0x{offset:02x}+{size} exceeds entry size {}",
                        module.entry.size
                    ),
                ));
            }
        }
    }

    // Field rule 13: partially overlapping byte ranges are warnings.
    report_partial_overlaps(file, module, report);
}

/// Identical storage ranges are allowed; partial overlaps warn (they are
/// usually offset mistakes). Sections never participate.
fn report_partial_overlaps(file: &str, module: &ModuleFile, report: &mut ValidationReport) {
    let stored: Vec<(&Field, u64, u64)> = module
        .fields
        .iter()
        .filter_map(|f| f.storage().map(|(offset, size)| (f, offset, size)))
        .collect();

    for (i, (left, left_offset, left_size)) in stored.iter().enumerate() {
        for (right, right_offset, right_size) in stored.iter().skip(i + 1) {
            let identical = left_offset == right_offset && left_size == right_size;
            let overlaps = left_offset < &(right_offset + right_size)
                && right_offset < &(left_offset + left_size);
            if overlaps && !identical {
                report.warnings.push(Issue {
                    file: file.to_string(),
                    module_id: Some(module.id.clone()),
                    field_id: Some(left.id().to_string()),
                    message: format!(
                        "fields '{}' (offset 0x{left_offset:02x}, size {left_size}) and '{}' \
                         (offset 0x{right_offset:02x}, size {right_size}) partially overlap",
                        left.id(),
                        right.id()
                    ),
                });
            }
        }
    }
}

/// Sidecar validation rules 5 and 6: duplicate values and empty labels are
/// warnings. Reported once per file.
pub fn report_sidecar_content(file: &str, items: &[SidecarItem], report: &mut ValidationReport) {
    let mut counts: BTreeMap<u64, usize> = BTreeMap::new();
    for item in items {
        *counts.entry(item.value).or_default() += 1;
    }
    let duplicates: Vec<String> = counts
        .iter()
        .filter(|(_, &count)| count > 1)
        .map(|(value, count)| format!("0x{value:02x} ({count} times)"))
        .collect();
    if !duplicates.is_empty() {
        report.warnings.push(Issue {
            file: file.to_string(),
            module_id: None,
            field_id: None,
            message: format!("duplicate values (aliases): {}", duplicates.join(", ")),
        });
    }

    let empty_labels = items.iter().filter(|item| item.label.is_empty()).count();
    if empty_labels > 0 {
        report.warnings.push(Issue {
            file: file.to_string(),
            module_id: None,
            field_id: None,
            message: format!("{empty_labels} item(s) have empty labels"),
        });
    }
}

/// Sidecar validation rule 7: option values must fit every dropdown size
/// that references the option file. Violations are errors.
pub fn report_option_size_overflow(
    file: &str,
    module: &ModuleFile,
    field: &Field,
    options_file: &str,
    items: &[SidecarItem],
    report: &mut ValidationReport,
) {
    let Some((_, size)) = field.storage() else {
        return;
    };
    let max = match size {
        1 => 0xff,
        2 => 0xffff,
        3 => 0xff_ffff,
        _ => 0xffff_ffff_u64,
    };
    let over: Vec<u64> = items.iter().map(|i| i.value).filter(|v| *v > max).collect();
    if !over.is_empty() {
        report.errors.push(Issue {
            file: file.to_string(),
            module_id: Some(module.id.clone()),
            field_id: Some(field.id().to_string()),
            message: format!(
                "{options_file}: {} option value(s) exceed the {size}-byte dropdown range \
                 (max 0x{max:x}), first 0x{:x}",
                over.len(),
                over[0]
            ),
        });
    }
}

/// Sidecar validation rule 8: entry label values at or above a referencing
/// module's declared `entry.count` are warnings. The spec explicitly
/// supports sharing one entry file between modules with different counts,
/// so these stay informational. Header-driven modules without an expected
/// count have no static count to check against.
pub fn report_entry_count_overflow(
    file: &str,
    module: &ModuleFile,
    labels: &[SidecarItem],
    report: &mut ValidationReport,
) {
    let count = match module.entry.count {
        CountSpec::Fixed(count) => count,
        CountSpec::Header {
            expected: Some(count),
        } => count,
        CountSpec::Header { expected: None } => return,
    };
    let over = labels.iter().filter(|item| item.value >= count).count();
    if over > 0 {
        report.warnings.push(Issue {
            file: file.to_string(),
            module_id: Some(module.id.clone()),
            field_id: None,
            message: format!(
                "{over} entry label value(s) at or above the declared entry count {count} \
                 are never displayed"
            ),
        });
    }
}
