use std::path::Path;

use super::*;

// --- fixtures -----------------------------------------------------------------

/// Writes a module tree into a temp directory.
fn write_set(files: &[(&str, &str)]) -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    for (path, content) in files {
        let full = dir.path().join(path);
        std::fs::create_dir_all(full.parent().unwrap()).unwrap();
        std::fs::write(full, content).unwrap();
    }
    dir
}

fn load(files: &[(&str, &str)]) -> (ModuleSet, ValidationReport) {
    let dir = write_set(files);
    load_module_set(dir.path()).unwrap()
}

const OPTIONS: &str = "[ { value: 0x00, label: 'No' }, { value: 0x01, label: 'Yes' } ]";
const ENTRIES: &str = "[ { value: 0x00, label: 'First' }, { value: 0x01, label: 'Second' } ]";

/// A valid module: 4 entries of 8 bytes with a dropdown, a uint, and a section.
fn valid_module(id: &str) -> String {
    format!(
        "{{
            schema_version: 1,
            id: '{id}',
            label: 'Test',
            files: ['battle/battle_data_release.dat'],
            base_offset: 0x100,
            entry: {{ count: 4, size: 8, labels_file: 'entries/test.json5' }},
            fields: [
                {{ id: 'kind', label: 'Kind', offset: 0, size: 1, type: 'dropdown', options_file: 'options/no_yes.json5' }},
                {{ id: 'power', label: 'Power', offset: 1, size: 2, type: 'uint' }},
                {{ id: 'general', label: 'General', type: 'section' }}
            ]
        }}"
    )
}

fn valid_set(id: &str) -> Vec<(&'static str, String)> {
    vec![
        ("test_module.json5", valid_module(id)),
        ("options/no_yes.json5", OPTIONS.to_string()),
        ("entries/test.json5", ENTRIES.to_string()),
    ]
}

fn load_owned(files: &[(&str, String)]) -> (ModuleSet, ValidationReport) {
    let borrowed: Vec<(&str, &str)> = files.iter().map(|(p, c)| (*p, c.as_str())).collect();
    load(&borrowed)
}

// --- happy path ----------------------------------------------------------------

#[test]
fn valid_set_loads_without_issues() {
    let (set, report) = load_owned(&valid_set("test_module"));

    assert_eq!(report.errors, Vec::new());
    assert_eq!(report.warnings, Vec::new());
    assert!(report.is_valid());
    assert_eq!(set.modules.len(), 1);

    let module = set.find("test_module").expect("module should be loaded");
    assert_eq!(module.file, "test_module.json5");
    assert_eq!(module.entry_labels.as_ref().unwrap().len(), 2);
    assert_eq!(module.options_for("options/no_yes.json5").unwrap().len(), 2);
}

#[test]
fn shared_sidecars_are_loaded_once_and_shared() {
    let module_b = valid_module("module_b").replace("'test_module'", "'module_b'");
    let mut files = valid_set("module_a");
    files[0] = ("module_a.json5", valid_module("module_a"));
    files.push(("module_b.json5", module_b));
    let (set, report) = load_owned(&files);

    assert!(report.is_valid());
    assert_eq!(set.modules.len(), 2);
    let a = set.find("module_a").unwrap();
    let b = set.find("module_b").unwrap();
    assert!(Arc::ptr_eq(
        a.options_for("options/no_yes.json5").unwrap(),
        b.options_for("options/no_yes.json5").unwrap()
    ));
}

// --- module-level errors -----------------------------------------------------------

#[test]
fn parse_errors_are_reported_and_other_modules_still_load() {
    let mut files = valid_set("good_module");
    files.push(("broken.json5", "{ not valid json5".to_string()));
    let (set, report) = load_owned(&files);

    assert_eq!(set.modules.len(), 1);
    assert_eq!(report.errors.len(), 1);
    assert_eq!(report.errors[0].file, "broken.json5");
    assert_eq!(report.errors[0].module_id, None);
}

#[test]
fn duplicate_module_ids_are_rejected() {
    let mut files = valid_set("same_id");
    files[0] = ("a_first.json5", valid_module("same_id"));
    files.push(("b_second.json5", valid_module("same_id")));
    let (set, report) = load_owned(&files);

    // The first file (alphabetically) wins; the second is excluded.
    assert_eq!(set.modules.len(), 1);
    assert_eq!(set.modules[0].file, "a_first.json5");
    assert_eq!(report.errors.len(), 1);
    assert_eq!(report.errors[0].file, "b_second.json5");
    assert!(report.errors[0]
        .message
        .contains("already used by a_first.json5"));
}

#[test]
fn invalid_module_and_field_ids_are_rejected() {
    for bad_id in ["Bad", "9lives", "has-dash", "_leading", ""] {
        let mut files = valid_set("placeholder");
        let text = valid_module("placeholder").replace("'placeholder'", &format!("'{bad_id}'"));
        files[0] = ("test_module.json5", text);
        let (set, report) = load_owned(&files);
        assert_eq!(set.modules.len(), 0, "id {bad_id:?} should be rejected");
        assert!(report.errors.iter().any(|i| i.message.contains("[a-z]")));
    }

    let mut files = valid_set("test_module");
    files[0] = (
        "test_module.json5",
        valid_module("test_module").replace("id: 'power'", "id: 'Power'"),
    );
    let (set, report) = load_owned(&files);
    assert_eq!(set.modules.len(), 0);
    assert!(report
        .errors
        .iter()
        .any(|i| i.field_id.as_deref() == Some("Power")));
}

#[test]
fn files_pattern_rules_are_enforced() {
    for (pattern, reason) in [
        ("/abs/path.dat", "begin with '/'"),
        ("battle/../secret.dat", "'..'"),
        ("battle\\\\windows.dat", "POSIX"),
        ("battle/dir/", "directory"),
        ("battle//double.dat", "empty path segments"),
        ("", "empty"),
    ] {
        let mut files = valid_set("test_module");
        let text = valid_module("test_module")
            .replace("'battle/battle_data_release.dat'", &format!("'{pattern}'"));
        files[0] = ("test_module.json5", text);
        let (set, report) = load_owned(&files);
        assert_eq!(
            set.modules.len(),
            0,
            "pattern {pattern:?} should be rejected"
        );
        assert!(
            report.errors.iter().any(|i| i.message.contains(reason)),
            "expected reason {reason:?} for {pattern:?}, got {:?}",
            report.errors
        );
    }
}

#[test]
fn empty_files_list_is_rejected() {
    let mut files = valid_set("test_module");
    let text = valid_module("test_module")
        .replace("files: ['battle/battle_data_release.dat'],", "files: [],");
    files[0] = ("test_module.json5", text);
    let (set, report) = load_owned(&files);
    assert_eq!(set.modules.len(), 0);
    assert!(report.errors.iter().any(|i| i.message.contains("files")));
}

#[test]
fn empty_fields_list_is_rejected() {
    let (set, report) = load(&[(
        "test_module.json5",
        "{
            schema_version: 1,
            id: 'test_module',
            label: 'Test',
            files: ['a.dat'],
            base_offset: 0,
            entry: { count: 1, size: 8 },
            fields: []
        }",
    )]);
    assert_eq!(set.modules.len(), 0);
    assert!(report.errors.iter().any(|i| i.message.contains("fields")));
}

#[test]
fn duplicate_field_ids_are_rejected() {
    let mut files = valid_set("test_module");
    let text = valid_module("test_module").replace("id: 'power'", "id: 'kind'");
    files[0] = ("test_module.json5", text);
    let (set, report) = load_owned(&files);
    assert_eq!(set.modules.len(), 0);
    assert!(report
        .errors
        .iter()
        .any(|i| i.message.contains("not unique")));
}

#[test]
fn field_storage_must_fit_entry_size() {
    let mut files = valid_set("test_module");
    // Entry size is 8; offset 7 + size 2 exceeds it.
    let text = valid_module("test_module").replace("offset: 1, size: 2", "offset: 7, size: 2");
    files[0] = ("test_module.json5", text);
    let (set, report) = load_owned(&files);
    assert_eq!(set.modules.len(), 0);
    assert!(report
        .errors
        .iter()
        .any(|i| i.message.contains("exceeds entry size")));
}

// --- overlaps ---------------------------------------------------------------------

#[test]
fn partial_overlaps_warn_but_do_not_exclude() {
    let mut files = valid_set("test_module");
    // 'power' at offset 0 size 2 partially overlaps 'kind' at offset 0 size 1.
    let text = valid_module("test_module").replace("offset: 1, size: 2", "offset: 0, size: 2");
    files[0] = ("test_module.json5", text);
    let (set, report) = load_owned(&files);

    assert_eq!(set.modules.len(), 1);
    assert!(report.errors.is_empty());
    assert_eq!(report.warnings.len(), 1);
    assert!(report.warnings[0].message.contains("partially overlap"));
}

#[test]
fn identical_ranges_do_not_warn() {
    let mut files = valid_set("test_module");
    // Same offset and size as 'kind': an intentional alias view.
    let text = valid_module("test_module").replace("offset: 1, size: 2", "offset: 0, size: 1");
    files[0] = ("test_module.json5", text);
    let (_, report) = load_owned(&files);
    assert!(report.warnings.is_empty(), "{:?}", report.warnings);
}

// --- sidecar resolution --------------------------------------------------------------

#[test]
fn missing_sidecar_file_is_an_error() {
    let mut files = valid_set("test_module");
    files.remove(2); // drop entries/test.json5
    let (set, report) = load_owned(&files);
    assert_eq!(set.modules.len(), 0);
    assert!(report
        .errors
        .iter()
        .any(|i| i.file == "entries/test.json5" && i.message.contains("failed to read")));
}

#[test]
fn sidecar_parse_errors_are_attributed_to_the_sidecar() {
    let mut files = valid_set("test_module");
    files[1] = ("options/no_yes.json5", "{ not: 'an array' }".to_string());
    let (set, report) = load_owned(&files);
    assert_eq!(set.modules.len(), 0);
    assert!(report
        .errors
        .iter()
        .any(|i| i.file == "options/no_yes.json5"));
}

#[test]
fn sidecar_paths_escaping_the_module_tree_are_rejected() {
    for bad in [
        "../outside.json5",
        "/etc/options.json5",
        "options/../../x.json5",
    ] {
        let mut files = valid_set("test_module");
        let text = valid_module("test_module").replace("options/no_yes.json5", bad);
        files[0] = ("test_module.json5", text);
        let (set, report) = load_owned(&files);
        assert_eq!(set.modules.len(), 0, "path {bad:?} should be rejected");
        assert!(report
            .errors
            .iter()
            .any(|i| i.message.contains("invalid sidecar path")));
    }
}

#[test]
fn sidecar_paths_normalize_dot_segments_within_the_tree() {
    let mut files = valid_set("test_module");
    let text =
        valid_module("test_module").replace("options/no_yes.json5", "./options/no_yes.json5");
    files[0] = ("test_module.json5", text);
    let (set, report) = load_owned(&files);
    assert!(report.is_valid(), "{:?}", report.errors);
    assert_eq!(
        set.modules[0]
            .options_for("./options/no_yes.json5")
            .unwrap()
            .len(),
        2
    );
}

// --- sidecar content rules ------------------------------------------------------------

#[test]
fn duplicate_option_values_warn_once_per_file() {
    let module_b = valid_module("module_b");
    let mut files = vec![
        ("module_a.json5", valid_module("module_a")),
        ("module_b.json5", module_b),
        (
            "options/no_yes.json5",
            "[ { value: 0, label: 'No' }, { value: 0, label: 'Alias' } ]".to_string(),
        ),
        ("entries/test.json5", ENTRIES.to_string()),
    ];
    files.swap(2, 3);
    let (set, report) = load_owned(&files);

    assert_eq!(set.modules.len(), 2);
    let dup_warnings: Vec<_> = report
        .warnings
        .iter()
        .filter(|i| i.message.contains("duplicate values"))
        .collect();
    assert_eq!(dup_warnings.len(), 1, "{:?}", report.warnings);
    assert_eq!(dup_warnings[0].file, "options/no_yes.json5");
}

#[test]
fn empty_labels_warn() {
    let mut files = valid_set("test_module");
    files[1] = (
        "options/no_yes.json5",
        "[ { value: 0, label: '' }, { value: 1, label: 'Yes' } ]".to_string(),
    );
    let (set, report) = load_owned(&files);
    assert_eq!(set.modules.len(), 1);
    assert!(report
        .warnings
        .iter()
        .any(|i| i.message.contains("empty labels")));
}

#[test]
fn option_values_must_fit_the_dropdown_size() {
    let mut files = valid_set("test_module");
    // 'kind' is a 1-byte dropdown; 0x100 does not fit.
    files[1] = (
        "options/no_yes.json5",
        "[ { value: 0x100, label: 'Too big' } ]".to_string(),
    );
    let (set, report) = load_owned(&files);
    assert_eq!(set.modules.len(), 0);
    let error = report
        .errors
        .iter()
        .find(|i| i.message.contains("exceed"))
        .expect("size overflow should be an error");
    assert_eq!(error.field_id.as_deref(), Some("kind"));
}

#[test]
fn entry_labels_at_or_above_fixed_count_warn() {
    let mut files = valid_set("test_module");
    // Count is 4; values 4 and 9 are out of range, 3 is fine.
    files[2] = (
        "entries/test.json5",
        "[ { value: 3, label: 'Ok' }, { value: 4, label: 'Over' }, { value: 9, label: 'Way over' } ]"
            .to_string(),
    );
    let (set, report) = load_owned(&files);
    assert_eq!(set.modules.len(), 1);
    let warning = report
        .warnings
        .iter()
        .find(|i| i.message.contains("never displayed"))
        .expect("entry overflow should warn");
    assert!(warning.message.contains('2'), "{}", warning.message);
    assert_eq!(warning.module_id.as_deref(), Some("test_module"));
}

#[test]
fn dynamic_count_modules_skip_the_entry_count_check() {
    let (set, report) = load(&[
        (
            "dynamic.json5",
            "{
                schema_version: 1,
                id: 'dynamic',
                label: 'Dynamic',
                files: ['battle/entry/entry_unit_*.dat'],
                base_offset: 0x30,
                entry: {
                    count_from: { base_offset: 0x20, offset: 0x04, size: 4, type: 'uint' },
                    size: 0xc4,
                    labels_file: 'entries/test.json5'
                },
                fields: [ { id: 'a', label: 'A', offset: 0, size: 1, type: 'uint' } ]
            }",
        ),
        (
            "entries/test.json5",
            "[ { value: 9999, label: 'High value' } ]",
        ),
    ]);
    assert_eq!(set.modules.len(), 1);
    assert!(report.warnings.is_empty(), "{:?}", report.warnings);
}

// --- non-module files ------------------------------------------------------------------

#[test]
fn subdirectories_and_non_json5_files_are_ignored() {
    let mut files = valid_set("test_module");
    files.push(("README.md", "not a module".to_string()));
    files.push(("options/extra.json5", OPTIONS.to_string()));
    let (set, report) = load_owned(&files);
    assert!(report.is_valid());
    assert_eq!(set.modules.len(), 1);
}

// --- golden test against the committed set ------------------------------------------------

/// The committed module set must load with zero errors and exactly the
/// expected warnings: the two known partial overlaps (spec "Duplicate and
/// Overlapping Storage"), alias values faithful to the Nightmare sources,
/// and the spec-blessed entries/class.json5 sharing case in menu_classmark.
#[test]
fn committed_module_set_is_valid() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../modules");
    let (set, report) = load_module_set(&dir).unwrap();

    assert_eq!(report.errors, Vec::new());
    assert_eq!(set.modules.len(), 24);

    let mut found: Vec<(&str, &str)> = report
        .warnings
        .iter()
        .map(|i| {
            let kind = if i.message.contains("partially overlap") {
                "overlap"
            } else if i.message.contains("duplicate values") {
                "aliases"
            } else if i.message.contains("never displayed") {
                "entry_count"
            } else {
                "unexpected"
            };
            (i.file.as_str(), kind)
        })
        .collect();
    found.sort();

    let mut expected = vec![
        ("battle_armament.json5", "overlap"),
        ("battle_class.json5", "overlap"),
        ("options/skill.json5", "aliases"),
        ("options/armament_set_ref.json5", "aliases"),
        ("options/class_set_ref.json5", "aliases"),
        ("menu_classmark.json5", "entry_count"),
    ];
    expected.sort();

    assert_eq!(found, expected, "warnings: {:#?}", report.warnings);
}

#[test]
fn committed_set_exposes_armament_with_sidecars() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../modules");
    let (set, _) = load_module_set(&dir).unwrap();

    let armament = set.find("battle_armament").expect("armament should load");
    assert_eq!(armament.entry_labels.as_ref().unwrap().len(), 761);
    assert!(armament.options_for("options/item_type.json5").is_some());

    // entries/class.json5 is shared between battle_class (256) and
    // menu_classmark (80); both must expose the same Arc.
    let class = set.find("battle_class").unwrap();
    let classmark = set.find("menu_classmark").unwrap();
    assert!(Arc::ptr_eq(
        class.entry_labels.as_ref().unwrap(),
        classmark.entry_labels.as_ref().unwrap()
    ));
}
