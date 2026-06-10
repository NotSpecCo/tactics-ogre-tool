use super::*;

// --- helpers -------------------------------------------------------------------

/// A minimal valid module with replaceable entry and fields fragments.
fn module_text(entry: &str, fields: &str) -> String {
    format!(
        "{{
            schema_version: 1,
            id: 'test_module',
            label: 'Test',
            files: ['battle/battle_data_release.dat'],
            base_offset: 0x100,
            entry: {entry},
            fields: [{fields}]
        }}"
    )
}

const ENTRY: &str = "{ count: 16, size: 32 }";
const FIELD_UINT: &str = "{ id: 'power', label: 'Power', offset: 0x00, size: 2, type: 'uint' }";

fn parse_ok(entry: &str, fields: &str) -> ModuleFile {
    parse_module(&module_text(entry, fields)).expect("module should parse")
}

fn parse_err(entry: &str, fields: &str) -> ParseError {
    parse_module(&module_text(entry, fields)).expect_err("module should be rejected")
}

fn assert_rejected_mentioning(text: &str, needle: &str) {
    let err = parse_module(text).expect_err("module should be rejected");
    let message = err.to_string();
    assert!(
        message.contains(needle),
        "error {message:?} should mention {needle:?}"
    );
}

// --- spec example --------------------------------------------------------------

/// The full module example from docs/MODULE-SPEC.md, verbatim.
const SPEC_EXAMPLE: &str = r#"
{
    schema_version: 1,
    id: 'battle_armament',
    label: 'Armament',
    notes: 'Weapon and armor stats.',

    source: {
        format: 'nightmare',
        path: 'battle/Armament.nmm',
        title: 'Tactics Ogre: Reborn ==> battle_data_release / pack ==> Armament (Equipment)'
    },

    files: ['battle/battle_data_release.dat'],

    base_offset: 0x00393f20,
    endian: 'little',

    entry: {
        header: true,
        count: 761,
        size: 152,
        labels_file: 'entries/armament.json5'
    },

    fields: [
        {
            id: 'item_type',
            label: 'Item Type',
            offset: 0x00,
            size: 1,
            type: 'dropdown',
            display: 'hex',
            options_file: 'options/item_type.json5'
        },
        {
            id: 'attack_power',
            label: 'Attack Power',
            offset: 0x04,
            size: 2,
            type: 'uint'
        },
        {
            id: 'weight',
            label: 'Weight',
            offset: 0x08,
            size: 2,
            type: 'int'
        },
        {
            id: 'unknown_0a',
            label: 'Unknown Bytes',
            offset: 0x0a,
            size: 4,
            type: 'bytes'
        },
        {
            id: 'name',
            label: 'Name',
            offset: 0x10,
            size: 16,
            type: 'text'
        },
        {
            id: 'general_flags',
            type: 'section',
            label: 'General Flags'
        }
    ]
}
"#;

#[test]
fn spec_example_module_parses() {
    let module = parse_module(SPEC_EXAMPLE).expect("spec example should parse");

    assert_eq!(module.schema_version, 1);
    assert_eq!(module.id, "battle_armament");
    assert_eq!(module.label, "Armament");
    assert_eq!(module.notes.as_deref(), Some("Weapon and armor stats."));
    assert_eq!(module.files, vec!["battle/battle_data_release.dat"]);
    assert_eq!(module.base_offset, 0x0039_3f20);
    assert_eq!(module.endian, Endian::Little);

    let source = module.source.expect("source should be present");
    assert_eq!(source.format.as_deref(), Some("nightmare"));
    assert_eq!(source.path.as_deref(), Some("battle/Armament.nmm"));
    assert!(source.title.unwrap().contains("Armament (Equipment)"));

    assert_eq!(
        module.entry.count,
        CountSpec::Header {
            expected: Some(761)
        }
    );
    assert_eq!(module.entry.size, 152);
    assert_eq!(
        module.entry.labels_file.as_deref(),
        Some("entries/armament.json5")
    );

    assert_eq!(module.fields.len(), 6);
    match &module.fields[0] {
        Field::Dropdown(f) => {
            assert_eq!(f.id, "item_type");
            assert_eq!(f.offset, 0x00);
            assert_eq!(f.size, 1);
            assert_eq!(f.display, DisplayFormat::Hex);
            assert_eq!(f.options_file, "options/item_type.json5");
        }
        other => panic!("expected dropdown, got {other:?}"),
    }
    match &module.fields[1] {
        Field::Uint(f) => {
            assert_eq!(f.size, 2);
            assert_eq!(f.display, DisplayFormat::Decimal);
            assert_eq!(f.endian, None);
        }
        other => panic!("expected uint, got {other:?}"),
    }
    assert!(matches!(&module.fields[2], Field::Int(f) if f.size == 2));
    assert!(matches!(&module.fields[3], Field::Bytes(f) if f.size == 4));
    assert!(matches!(&module.fields[4], Field::Text(f) if f.size == 16));
    match &module.fields[5] {
        Field::Section(f) => {
            assert_eq!(f.id, "general_flags");
            assert_eq!(f.label, "General Flags");
        }
        other => panic!("expected section, got {other:?}"),
    }
}

#[test]
fn spec_header_driven_glob_example_parses() {
    // The BattleUnit example from the spec's "Battle Unit Tables" section:
    // header-driven with no expected count.
    let module = parse_ok(
        "{
            header: true,
            size: 0xc4,
            labels_file: null
        }",
        FIELD_UINT,
    );

    assert_eq!(module.entry.count, CountSpec::Header { expected: None });
    assert_eq!(module.entry.size, 0xc4);
    assert_eq!(module.entry.labels_file, None);
}

// --- defaults and optional keys --------------------------------------------------

#[test]
fn header_true_with_count_keeps_the_expected_count() {
    let module = parse_ok("{ header: true, count: 16, size: 32 }", FIELD_UINT);
    assert_eq!(module.entry.count, CountSpec::Header { expected: Some(16) });
}

#[test]
fn header_false_is_equivalent_to_absent() {
    let explicit = parse_ok("{ header: false, count: 16, size: 32 }", FIELD_UINT);
    let absent = parse_ok("{ count: 16, size: 32 }", FIELD_UINT);
    assert_eq!(explicit.entry.count, CountSpec::Fixed(16));
    assert_eq!(explicit.entry, absent.entry);
}

#[test]
fn module_endian_defaults_to_little_and_accepts_big() {
    assert_eq!(parse_ok(ENTRY, FIELD_UINT).endian, Endian::Little);

    let text = module_text(ENTRY, FIELD_UINT).replace("entry:", "endian: 'big', entry:");
    assert_eq!(parse_module(&text).unwrap().endian, Endian::Big);
}

#[test]
fn field_endian_override_parses_on_uint_int_dropdown() {
    let module = parse_ok(
        ENTRY,
        "{ id: 'a', label: 'A', offset: 0, size: 2, type: 'uint', endian: 'big' },
         { id: 'b', label: 'B', offset: 2, size: 2, type: 'int', endian: 'big' },
         { id: 'c', label: 'C', offset: 4, size: 2, type: 'dropdown', endian: 'big', options_file: 'options/x.json5' }",
    );
    assert!(matches!(&module.fields[0], Field::Uint(f) if f.endian == Some(Endian::Big)));
    assert!(matches!(&module.fields[1], Field::Int(f) if f.endian == Some(Endian::Big)));
    assert!(matches!(&module.fields[2], Field::Dropdown(f) if f.endian == Some(Endian::Big)));
}

#[test]
fn labels_file_null_and_absent_are_equivalent() {
    let with_null = parse_ok("{ count: 1, size: 4, labels_file: null }", FIELD_UINT);
    let absent = parse_ok("{ count: 1, size: 4 }", FIELD_UINT);
    assert_eq!(with_null.entry.labels_file, None);
    assert_eq!(absent.entry.labels_file, None);
}

#[test]
fn notes_parse_on_fields_and_sections() {
    let module = parse_ok(
        ENTRY,
        "{ id: 'a', label: 'A', notes: 'Stored note.', offset: 0, size: 1, type: 'uint' },
         { id: 's', label: 'S', notes: 'Section note.', type: 'section' }",
    );
    assert_eq!(module.fields[0].notes(), Some("Stored note."));
    assert_eq!(module.fields[1].notes(), Some("Section note."));
}

#[test]
fn source_is_optional() {
    assert_eq!(parse_ok(ENTRY, FIELD_UINT).source, None);
}

#[test]
fn three_byte_uint_and_dropdown_parse() {
    let module = parse_ok(
        ENTRY,
        "{ id: 'a', label: 'A', offset: 0, size: 3, type: 'uint', display: 'hex' },
         { id: 'b', label: 'B', offset: 3, size: 3, type: 'dropdown', options_file: 'options/x.json5' }",
    );
    assert_eq!(module.fields[0].storage(), Some((0, 3)));
    assert_eq!(module.fields[1].storage(), Some((3, 3)));
}

#[test]
fn bytes_and_text_accept_large_sizes() {
    let module = parse_ok(
        "{ count: 1, size: 512 }",
        "{ id: 'blob', label: 'Blob', offset: 0, size: 152, type: 'bytes' },
         { id: 'name', label: 'Name', offset: 152, size: 64, type: 'text' }",
    );
    assert_eq!(module.fields[0].storage(), Some((0, 152)));
    assert_eq!(module.fields[1].storage(), Some((152, 64)));
}

// --- JSON5 lexical behavior -------------------------------------------------------

#[test]
fn comments_are_ignored() {
    let text = format!(
        "{{
            // line comment
            schema_version: 1, /* block comment */
            id: 'test_module',
            label: 'Test',
            files: ['a.dat'],
            base_offset: 0,
            entry: {ENTRY},
            fields: [{FIELD_UINT}]
        }}"
    );
    assert!(parse_module(&text).is_ok());
}

#[test]
fn hex_and_decimal_literals_are_equivalent() {
    let hex = parse_ok("{ count: 0x10, size: 0x20 }", FIELD_UINT);
    let dec = parse_ok("{ count: 16, size: 32 }", FIELD_UINT);
    assert_eq!(hex.entry, dec.entry);
}

#[test]
fn leading_plus_is_permitted() {
    // The spec permits a leading + because standard JSON5 parsers accept it.
    let module = parse_ok("{ count: +16, size: +32 }", FIELD_UINT);
    assert_eq!(module.entry.count, CountSpec::Fixed(16));
    assert_eq!(module.entry.size, 32);
}

#[test]
fn double_quoted_strings_parse() {
    let text = module_text(ENTRY, FIELD_UINT).replace("'Test'", "\"Test\"");
    assert_eq!(parse_module(&text).unwrap().label, "Test");
}

// --- numeric rejections -------------------------------------------------------------

#[test]
fn string_where_integer_expected_is_rejected() {
    parse_err("{ count: '16', size: 32 }", FIELD_UINT);
    parse_err("{ count: 16, size: '0x20' }", FIELD_UINT);

    let text = module_text(ENTRY, FIELD_UINT).replace("base_offset: 0x100", "base_offset: '0x100'");
    parse_module(&text).expect_err("string base_offset should be rejected");
}

#[test]
fn fractional_numbers_are_rejected() {
    // The json5 crate would silently truncate these into integer targets,
    // so the numeric pre-pass must reject them per the spec Numeric Rules.
    parse_err("{ count: 16.5, size: 32 }", FIELD_UINT);
    parse_err("{ count: 16.0, size: 32 }", FIELD_UINT);
    parse_err("{ count: .5, size: 32 }", FIELD_UINT);
}

#[test]
fn exponent_numbers_are_rejected() {
    parse_err("{ count: 1e2, size: 32 }", FIELD_UINT);
    parse_err("{ count: 1E2, size: 32 }", FIELD_UINT);
    parse_err("{ count: 1e+2, size: 32 }", FIELD_UINT);
}

#[test]
fn negative_integers_are_rejected_where_unsigned() {
    let text = module_text(ENTRY, FIELD_UINT).replace("base_offset: 0x100", "base_offset: -1");
    parse_module(&text).expect_err("negative base_offset should be rejected");

    parse_err("{ count: -1, size: 32 }", FIELD_UINT);

    let fields = "{ id: 'a', label: 'A', offset: -4, size: 2, type: 'uint' }";
    parse_err(ENTRY, fields);
}

#[test]
fn infinity_and_nan_are_rejected() {
    parse_err("{ count: Infinity, size: 32 }", FIELD_UINT);
    parse_err("{ count: NaN, size: 32 }", FIELD_UINT);
}

#[test]
fn booleans_arrays_and_objects_are_rejected_where_integers_expected() {
    parse_err("{ count: true, size: 32 }", FIELD_UINT);
    parse_err("{ count: [16], size: 32 }", FIELD_UINT);
    parse_err("{ count: { value: 16 }, size: 32 }", FIELD_UINT);
}

// --- shape rejections -----------------------------------------------------------------

#[test]
fn schema_version_other_than_1_is_rejected() {
    let text = module_text(ENTRY, FIELD_UINT).replace("schema_version: 1", "schema_version: 2");
    assert_rejected_mentioning(&text, "schema_version");
}

#[test]
fn entry_requires_count_unless_header() {
    let err = parse_err("{ size: 8 }", FIELD_UINT);
    assert!(err.to_string().contains("required unless"));

    let err = parse_err("{ header: false, size: 8 }", FIELD_UINT);
    assert!(err.to_string().contains("required unless"));
}

#[test]
fn zero_count_and_zero_sizes_are_rejected() {
    parse_err("{ count: 0, size: 32 }", FIELD_UINT);
    parse_err("{ header: true, count: 0, size: 32 }", FIELD_UINT);
    parse_err("{ count: 16, size: 0 }", FIELD_UINT);

    let fields = "{ id: 'a', label: 'A', offset: 0, size: 0, type: 'uint' }";
    parse_err(ENTRY, fields);
}

#[test]
fn header_must_be_a_boolean() {
    parse_err("{ header: 1, size: 8 }", FIELD_UINT);
    parse_err("{ header: 'true', size: 8 }", FIELD_UINT);
}

#[test]
fn count_from_is_no_longer_a_valid_key() {
    // The pre-header spec revision used a count_from block; it must now be
    // rejected as an unknown key so stale modules fail loudly.
    parse_err(
        "{ count_from: { offset: 0, size: 4, type: 'uint' }, size: 8 }",
        FIELD_UINT,
    );
}

// --- unknown and missing keys ------------------------------------------------------------

#[test]
fn unknown_keys_are_rejected_everywhere() {
    let top =
        module_text(ENTRY, FIELD_UINT).replace("schema_version: 1", "schema_version: 1, extra: 1");
    parse_module(&top).expect_err("unknown top-level key should be rejected");

    parse_err("{ count: 16, size: 32, extra: 1 }", FIELD_UINT);
    parse_err(
        "{ header: true, count: 16, size: 32, extra: 1 }",
        FIELD_UINT,
    );

    let fields = "{ id: 'a', label: 'A', offset: 0, size: 2, type: 'uint', extra: 1 }";
    parse_err(ENTRY, fields);

    let source = module_text(ENTRY, FIELD_UINT).replace(
        "entry:",
        "source: { format: 'nightmare', custom: true }, entry:",
    );
    parse_module(&source).expect_err("unknown source key should be rejected");
}

#[test]
fn missing_required_module_keys_are_rejected() {
    for key in ["schema_version: 1,", "id: 'test_module',", "label: 'Test',"] {
        let text = module_text(ENTRY, FIELD_UINT).replace(key, "");
        parse_module(&text).expect_err("missing required key should be rejected");
    }

    let no_files =
        module_text(ENTRY, FIELD_UINT).replace("files: ['battle/battle_data_release.dat'],", "");
    parse_module(&no_files).expect_err("missing files should be rejected");

    let no_base = module_text(ENTRY, FIELD_UINT).replace("base_offset: 0x100,", "");
    parse_module(&no_base).expect_err("missing base_offset should be rejected");
}

#[test]
fn missing_field_keys_are_rejected() {
    parse_err(ENTRY, "{ label: 'A', offset: 0, size: 2, type: 'uint' }");
    parse_err(ENTRY, "{ id: 'a', offset: 0, size: 2, type: 'uint' }");
    parse_err(ENTRY, "{ id: 'a', label: 'A', offset: 0, size: 2 }");

    let err = parse_err(ENTRY, "{ id: 'a', label: 'A', size: 2, type: 'uint' }");
    assert!(err.to_string().contains("offset"));
    let err = parse_err(ENTRY, "{ id: 'a', label: 'A', offset: 0, type: 'uint' }");
    assert!(err.to_string().contains("size"));
}

// --- per-type field key legality ------------------------------------------------------------

#[test]
fn section_fields_must_not_carry_storage_keys() {
    for extra in [
        "offset: 0",
        "size: 1",
        "display: 'hex'",
        "endian: 'big'",
        "options_file: 'options/x.json5'",
    ] {
        let fields = format!("{{ id: 's', label: 'S', type: 'section', {extra} }}");
        let err = parse_err(ENTRY, &fields);
        assert!(
            err.to_string().contains("section"),
            "error should mention section: {err}"
        );
    }
}

#[test]
fn integer_size_sets_are_enforced() {
    parse_err(
        ENTRY,
        "{ id: 'a', label: 'A', offset: 0, size: 5, type: 'uint' }",
    );
    parse_err(
        ENTRY,
        "{ id: 'a', label: 'A', offset: 0, size: 3, type: 'int' }",
    );
    parse_err(
        ENTRY,
        "{ id: 'a', label: 'A', offset: 0, size: 8, type: 'dropdown', options_file: 'options/x.json5' }",
    );
}

#[test]
fn dropdown_requires_options_file() {
    let err = parse_err(
        ENTRY,
        "{ id: 'a', label: 'A', offset: 0, size: 1, type: 'dropdown' }",
    );
    assert!(err.to_string().contains("options_file"));
}

#[test]
fn options_file_is_rejected_on_non_dropdown_fields() {
    for type_name in ["uint", "int", "bytes", "text"] {
        let fields = format!(
            "{{ id: 'a', label: 'A', offset: 0, size: 2, type: '{type_name}', options_file: 'options/x.json5' }}"
        );
        parse_err(ENTRY, &fields);
    }
}

#[test]
fn display_is_rejected_on_int_bytes_and_text() {
    for type_name in ["int", "bytes", "text"] {
        let fields = format!(
            "{{ id: 'a', label: 'A', offset: 0, size: 2, type: '{type_name}', display: 'hex' }}"
        );
        parse_err(ENTRY, &fields);
    }
}

#[test]
fn endian_is_rejected_on_bytes_and_text() {
    for type_name in ["bytes", "text"] {
        let fields = format!(
            "{{ id: 'a', label: 'A', offset: 0, size: 4, type: '{type_name}', endian: 'big' }}"
        );
        parse_err(ENTRY, &fields);
    }
}

#[test]
fn invalid_enum_values_are_rejected() {
    let endian = module_text(ENTRY, FIELD_UINT).replace("entry:", "endian: 'middle', entry:");
    parse_module(&endian).expect_err("invalid endian should be rejected");

    parse_err(
        ENTRY,
        "{ id: 'a', label: 'A', offset: 0, size: 2, type: 'uint', display: 'octal' }",
    );
    parse_err(
        ENTRY,
        "{ id: 'a', label: 'A', offset: 0, size: 4, type: 'float' }",
    );
}

// --- field accessors ---------------------------------------------------------------------

#[test]
fn field_accessors_expose_common_properties() {
    let module = parse_module(SPEC_EXAMPLE).unwrap();

    let dropdown = &module.fields[0];
    assert_eq!(dropdown.id(), "item_type");
    assert_eq!(dropdown.label(), "Item Type");
    assert_eq!(dropdown.type_name(), "dropdown");
    assert_eq!(dropdown.options_file(), Some("options/item_type.json5"));
    assert!(!dropdown.is_section());

    let section = &module.fields[5];
    assert!(section.is_section());
    assert_eq!(section.storage(), None);
    assert_eq!(section.options_file(), None);
    assert_eq!(section.type_name(), "section");
}

// --- sidecars -------------------------------------------------------------------------------

#[test]
fn spec_options_example_parses() {
    let items = parse_sidecar(
        "[
            { value: 0x00, label: 'None' },
            { value: 0x01, label: 'Sword', notes: 'Standard one-handed blade' },
            { value: 0x02, label: 'Axe' },
            { value: 0x03, label: 'Spear' }
        ]",
    )
    .expect("options example should parse");

    assert_eq!(items.len(), 4);
    assert_eq!(
        items[0],
        SidecarItem {
            value: 0,
            label: "None".into(),
            notes: None
        }
    );
    assert_eq!(items[1].notes.as_deref(), Some("Standard one-handed blade"));
    assert_eq!(items[3].value, 3);
}

#[test]
fn sidecar_preserves_file_order_and_allows_duplicates_and_gaps() {
    let items = parse_sidecar(
        "[
            { value: 0x05, label: 'Second alias' },
            { value: 0x05, label: 'First alias' },
            { value: 0x00, label: 'Out of order' }
        ]",
    )
    .unwrap();
    assert_eq!(
        items.iter().map(|i| i.value).collect::<Vec<_>>(),
        vec![5, 5, 0]
    );
}

#[test]
fn empty_sidecar_array_parses() {
    assert_eq!(parse_sidecar("[]").unwrap(), Vec::<SidecarItem>::new());
}

#[test]
fn sidecar_top_level_must_be_an_array() {
    parse_sidecar("{ value: 0, label: 'X' }").expect_err("object should be rejected");
    parse_sidecar("'just a string'").expect_err("string should be rejected");
}

#[test]
fn sidecar_items_require_value_and_label() {
    parse_sidecar("[{ label: 'X' }]").expect_err("missing value should be rejected");
    parse_sidecar("[{ value: 0 }]").expect_err("missing label should be rejected");
}

#[test]
fn sidecar_values_must_be_non_negative_integers() {
    parse_sidecar("[{ value: -1, label: 'X' }]").expect_err("negative value should be rejected");
    parse_sidecar("[{ value: 1.5, label: 'X' }]").expect_err("fractional value should be rejected");
    parse_sidecar("[{ value: '1', label: 'X' }]").expect_err("string value should be rejected");
}

#[test]
fn sidecar_labels_must_be_strings() {
    parse_sidecar("[{ value: 0, label: 5 }]").expect_err("numeric label should be rejected");
    parse_sidecar("[{ value: 0, label: null }]").expect_err("null label should be rejected");
}

#[test]
fn sidecar_unknown_keys_are_rejected() {
    parse_sidecar("[{ value: 0, label: 'X', color: 'red' }]")
        .expect_err("unknown key should be rejected");
}

#[test]
fn sidecar_empty_labels_parse() {
    // Empty labels are a validation warning, not a parse error.
    let items = parse_sidecar("[{ value: 0, label: '' }]").unwrap();
    assert_eq!(items[0].label, "");
}

#[test]
fn sidecar_comments_are_ignored() {
    let items = parse_sidecar(
        "[
            // weapon kinds
            { value: 0x00, label: 'None' } /* trailing */
        ]",
    )
    .unwrap();
    assert_eq!(items.len(), 1);
}

// --- error formatting -------------------------------------------------------------------------

#[test]
fn shape_errors_carry_context() {
    let err = parse_err("{ count: 0, size: 32 }", FIELD_UINT);
    match &err {
        ParseError::Shape { context, .. } => assert_eq!(context, "entry.count"),
        other => panic!("expected shape error, got {other:?}"),
    }
    assert!(err.to_string().contains("entry.count"));

    let err = parse_err(
        ENTRY,
        "{ id: 'bad_one', label: 'A', offset: 0, size: 9, type: 'uint' }",
    );
    assert!(err.to_string().contains("bad_one"));
}

#[test]
fn syntax_errors_are_reported_as_syntax() {
    let err = parse_module("{ this is not json5").expect_err("garbage should be rejected");
    assert!(matches!(err, ParseError::Syntax(_)));
}

// --- committed module set ----------------------------------------------------------------------

/// Every file the converter generated must parse with this parser. The full
/// validation golden test lands with the loader; this catches converter and
/// parser disagreeing on the format itself.
#[test]
fn committed_module_set_parses() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../modules");
    assert!(root.is_dir(), "expected generated module set at {root:?}");

    let mut modules = 0;
    let mut sidecars = 0;
    for entry in std::fs::read_dir(&root).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_some_and(|e| e == "json5") {
            let text = std::fs::read_to_string(&path).unwrap();
            parse_module(&text).unwrap_or_else(|e| panic!("{path:?} failed to parse: {e}"));
            modules += 1;
        }
    }
    for dir in ["options", "entries"] {
        for entry in std::fs::read_dir(root.join(dir)).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().is_some_and(|e| e == "json5") {
                let text = std::fs::read_to_string(&path).unwrap();
                parse_sidecar(&text).unwrap_or_else(|e| panic!("{path:?} failed to parse: {e}"));
                sidecars += 1;
            }
        }
    }

    assert_eq!(modules, 24, "expected 24 module files");
    assert_eq!(sidecars, 84 + 18, "expected 84 option and 18 entry files");
}
