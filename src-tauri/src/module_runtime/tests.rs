use super::*;
use crate::module_spec::parse_module;

// --- fixtures --------------------------------------------------------------------

/// Builds a module via the public parser so fixtures stay spec-shaped.
fn module(endian: &str, base_offset: u64, entry: &str, fields: &str) -> ModuleFile {
    parse_module(&format!(
        "{{
            schema_version: 1,
            id: 'test_module',
            label: 'Test',
            files: ['battle/test.dat'],
            base_offset: {base_offset},
            endian: '{endian}',
            entry: {entry},
            fields: [{fields}]
        }}"
    ))
    .expect("test module should parse")
}

/// A module with 2 entries of 8 bytes starting at offset 4, one stored
/// field of the given type fragment at offset 0.
fn single_field_module(endian: &str, field: &str) -> ModuleFile {
    module(endian, 4, "{ count: 2, size: 8 }", field)
}

/// Payload: 4 prefix bytes, then two 8-byte entries with recognizable bytes.
fn payload() -> Vec<u8> {
    let mut p = vec![0xAA; 4]; // prefix, not part of the table
    p.extend([0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]); // entry 0
    p.extend([0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18]); // entry 1
    p
}

fn read(module: &ModuleFile, payload: &[u8], index: u64) -> FieldValue {
    read_field(payload, module, index, &module.fields[0]).expect("read should succeed")
}

// --- resolve_count ------------------------------------------------------------------

#[test]
fn fixed_count_within_bounds_resolves() {
    let m = single_field_module(
        "little",
        "{ id: 'a', label: 'A', offset: 0, size: 1, type: 'uint' }",
    );
    assert_eq!(resolve_count(&payload(), &m), Ok(2));
}

#[test]
fn fixed_count_table_exceeding_payload_is_rejected() {
    let m = module(
        "little",
        4,
        "{ count: 3, size: 8 }",
        "{ id: 'a', label: 'A', offset: 0, size: 1, type: 'uint' }",
    );
    assert_eq!(
        resolve_count(&payload(), &m),
        Err(RuntimeError::TableOutOfBounds {
            required: 4 + 3 * 8,
            payload_len: 20
        })
    );
}

#[test]
fn count_from_reads_little_and_big_endian() {
    // Count lives at base_offset 2 + offset 2, size 2; table at 8 with 1-byte entries.
    let entry = "{ count_from: { base_offset: 2, offset: 2, size: 2, type: 'uint' }, size: 1 }";
    let field = "{ id: 'a', label: 'A', offset: 0, size: 1, type: 'uint' }";

    let mut p = vec![0u8; 8];
    p[4] = 0x03; // LE 0x0003
    p.extend([0x55; 3]); // exactly 3 one-byte entries at offset 8
    let m = module("little", 8, entry, field);
    assert_eq!(resolve_count(&p, &m), Ok(3));

    let mut p = vec![0u8; 8];
    p[5] = 0x03; // BE 0x0003
    p.extend([0x55; 3]);
    let m = module("big", 8, entry, field);
    assert_eq!(resolve_count(&p, &m), Ok(3));
}

#[test]
fn count_from_endian_override_beats_module_endian() {
    let entry = "{ count_from: { offset: 0, size: 2, type: 'uint', endian: 'big' }, size: 1 }";
    let field = "{ id: 'a', label: 'A', offset: 0, size: 1, type: 'uint' }";
    let m = module("little", 2, entry, field);

    let mut p = vec![0x00, 0x02]; // BE 2
    p.extend([0x55; 2]);
    assert_eq!(resolve_count(&p, &m), Ok(2));
}

#[test]
fn count_from_base_offset_defaults_to_zero() {
    let entry = "{ count_from: { offset: 1, size: 1, type: 'uint' }, size: 1 }";
    let field = "{ id: 'a', label: 'A', offset: 0, size: 1, type: 'uint' }";
    let m = module("little", 2, entry, field);
    let p = vec![0xAA, 0x02, 0x55, 0x55];
    assert_eq!(resolve_count(&p, &m), Ok(2));
}

#[test]
fn count_from_region_outside_payload_is_rejected() {
    let entry = "{ count_from: { offset: 6, size: 4, type: 'uint' }, size: 1 }";
    let field = "{ id: 'a', label: 'A', offset: 0, size: 1, type: 'uint' }";
    let m = module("little", 0, entry, field);
    assert_eq!(
        resolve_count(&[0u8; 8], &m),
        Err(RuntimeError::CountRegionOutOfBounds {
            start: 6,
            size: 4,
            payload_len: 8
        })
    );
}

#[test]
fn count_from_resolved_table_outside_payload_is_rejected() {
    let entry = "{ count_from: { offset: 0, size: 1, type: 'uint' }, size: 4 }";
    let field = "{ id: 'a', label: 'A', offset: 0, size: 1, type: 'uint' }";
    let m = module("little", 1, entry, field);
    // Count byte says 200 entries of 4 bytes; payload is 9 bytes.
    let p = vec![200, 0, 0, 0, 0, 0, 0, 0, 0];
    assert_eq!(
        resolve_count(&p, &m),
        Err(RuntimeError::TableOutOfBounds {
            required: 1 + 200 * 4,
            payload_len: 9
        })
    );
}

// --- reading integers ------------------------------------------------------------------

#[test]
fn uint_reads_all_sizes_little_endian() {
    for (size, expected) in [(1, 0x01), (2, 0x0201), (3, 0x030201), (4, 0x04030201u64)] {
        let m = single_field_module(
            "little",
            &format!("{{ id: 'a', label: 'A', offset: 0, size: {size}, type: 'uint' }}"),
        );
        assert_eq!(
            read(&m, &payload(), 0),
            FieldValue::Uint(expected),
            "size {size}"
        );
    }
}

#[test]
fn uint_reads_all_sizes_big_endian() {
    for (size, expected) in [(1, 0x01), (2, 0x0102), (3, 0x010203), (4, 0x01020304u64)] {
        let m = single_field_module(
            "big",
            &format!("{{ id: 'a', label: 'A', offset: 0, size: {size}, type: 'uint' }}"),
        );
        assert_eq!(
            read(&m, &payload(), 0),
            FieldValue::Uint(expected),
            "size {size}"
        );
    }
}

#[test]
fn field_endian_override_beats_module_endian() {
    let m = single_field_module(
        "little",
        "{ id: 'a', label: 'A', offset: 0, size: 2, type: 'uint', endian: 'big' }",
    );
    assert_eq!(read(&m, &payload(), 0), FieldValue::Uint(0x0102));
}

#[test]
fn second_entry_reads_from_the_next_stride() {
    let m = single_field_module(
        "little",
        "{ id: 'a', label: 'A', offset: 2, size: 2, type: 'uint' }",
    );
    // Entry 1 starts at 4 + 8 = 12; offset 2 -> bytes 0x13 0x14.
    assert_eq!(read(&m, &payload(), 1), FieldValue::Uint(0x1413));
}

#[test]
fn int_sign_extends_each_size() {
    let cases = [
        (vec![0xFF], 1, -1i64),
        (vec![0x7F], 1, 127),
        (vec![0x80], 1, -128),
        (vec![0x00, 0x80], 2, -32768),
        (vec![0xFF, 0x7F], 2, 32767),
        (vec![0xFF, 0xFF, 0xFF, 0xFF], 4, -1),
        (vec![0x00, 0x00, 0x00, 0x80], 4, i64::from(i32::MIN)),
    ];
    for (bytes, size, expected) in cases {
        let m = module(
            "little",
            0,
            "{ count: 1, size: 4 }",
            &format!("{{ id: 'a', label: 'A', offset: 0, size: {size}, type: 'int' }}"),
        );
        let mut p = bytes.clone();
        p.resize(4, 0);
        assert_eq!(
            read(&m, &p, 0),
            FieldValue::Int(expected),
            "bytes {bytes:?} size {size}"
        );
    }
}

#[test]
fn dropdown_reads_as_uint() {
    let m = single_field_module(
        "little",
        "{ id: 'a', label: 'A', offset: 0, size: 2, type: 'dropdown', options_file: 'options/x.json5' }",
    );
    assert_eq!(read(&m, &payload(), 0), FieldValue::Uint(0x0201));
}

#[test]
fn display_format_does_not_affect_stored_reads() {
    let plain = single_field_module(
        "little",
        "{ id: 'a', label: 'A', offset: 0, size: 2, type: 'uint' }",
    );
    let hex = single_field_module(
        "little",
        "{ id: 'a', label: 'A', offset: 0, size: 2, type: 'uint', display: 'hex' }",
    );
    assert_eq!(read(&plain, &payload(), 0), read(&hex, &payload(), 0));
}

#[test]
fn bytes_reads_the_exact_slice() {
    let m = single_field_module(
        "little",
        "{ id: 'a', label: 'A', offset: 2, size: 4, type: 'bytes' }",
    );
    assert_eq!(
        read(&m, &payload(), 0),
        FieldValue::Bytes(vec![0x03, 0x04, 0x05, 0x06])
    );
}

// --- reading text --------------------------------------------------------------------

fn text_module() -> ModuleFile {
    module(
        "little",
        0,
        "{ count: 1, size: 8 }",
        "{ id: 't', label: 'T', offset: 0, size: 8, type: 'text' }",
    )
}

#[test]
fn text_decodes_up_to_the_first_nul() {
    let p = *b"AB\0CDEFG";
    assert_eq!(read(&text_module(), &p, 0), FieldValue::Text("AB".into()));
}

#[test]
fn text_without_terminator_decodes_the_full_field() {
    let p = *b"ABCDEFGH";
    assert_eq!(
        read(&text_module(), &p, 0),
        FieldValue::Text("ABCDEFGH".into())
    );
}

#[test]
fn text_decodes_non_printable_bytes_as_replacement() {
    let p = [0x41, 0x09, 0x80, 0xFF, 0x42, 0x00, 0x00, 0x00];
    assert_eq!(
        read(&text_module(), &p, 0),
        FieldValue::Text("A\u{FFFD}\u{FFFD}\u{FFFD}B".into())
    );
}

// --- read errors ------------------------------------------------------------------------

#[test]
fn sections_have_no_readable_value() {
    let m = module(
        "little",
        4,
        "{ count: 2, size: 8 }",
        "{ id: 's', label: 'S', type: 'section' }",
    );
    assert_eq!(
        read_field(&payload(), &m, 0, &m.fields[0]),
        Err(RuntimeError::SectionHasNoValue {
            field_id: "s".into()
        })
    );
}

#[test]
fn index_at_or_above_count_is_rejected() {
    let m = single_field_module(
        "little",
        "{ id: 'a', label: 'A', offset: 0, size: 1, type: 'uint' }",
    );
    assert_eq!(
        read_field(&payload(), &m, 2, &m.fields[0]),
        Err(RuntimeError::IndexOutOfRange { index: 2, count: 2 })
    );
}

#[test]
fn field_extending_past_payload_is_rejected() {
    // Field exceeds entry.size (a validator error, but the runtime must
    // still bounds-check unvalidated modules): entry fits, field does not.
    let m = module(
        "little",
        16,
        "{ count: 1, size: 4 }",
        "{ id: 'a', label: 'A', offset: 2, size: 4, type: 'uint' }",
    );
    let p = vec![0u8; 20]; // table 16..20 fits; field 18..22 does not
    assert_eq!(
        read_field(&p, &m, 0, &m.fields[0]),
        Err(RuntimeError::FieldOutOfBounds {
            field_id: "a".into(),
            start: 18,
            size: 4,
            payload_len: 20
        })
    );
}

// --- read_record ----------------------------------------------------------------------------

#[test]
fn read_record_aligns_values_with_fields_and_skips_sections() {
    let m = module(
        "little",
        4,
        "{ count: 2, size: 8 }",
        "{ id: 'a', label: 'A', offset: 0, size: 1, type: 'uint' },
         { id: 's', label: 'S', type: 'section' },
         { id: 'b', label: 'B', offset: 1, size: 1, type: 'int' }",
    );
    let record = read_record(&payload(), &m, 0).unwrap();
    assert_eq!(
        record,
        vec![
            Some(FieldValue::Uint(0x01)),
            None,
            Some(FieldValue::Int(0x02)),
        ]
    );
}

// --- writing integers --------------------------------------------------------------------------

#[test]
fn uint_write_round_trips_every_size_and_endian() {
    for endian in ["little", "big"] {
        for size in [1u64, 2, 3, 4] {
            let m = single_field_module(
                endian,
                &format!("{{ id: 'a', label: 'A', offset: 0, size: {size}, type: 'uint' }}"),
            );
            let mut p = payload();
            let value = max_unsigned(size) - 1;
            write_field(&mut p, &m, 0, "a", &FieldValue::Uint(value)).unwrap();
            assert_eq!(
                read(&m, &p, 0),
                FieldValue::Uint(value),
                "size {size} {endian}"
            );
        }
    }
}

#[test]
fn uint_write_is_byte_exact_per_endian() {
    let m = single_field_module(
        "little",
        "{ id: 'a', label: 'A', offset: 0, size: 3, type: 'uint' }",
    );
    let mut p = payload();
    write_field(&mut p, &m, 0, "a", &FieldValue::Uint(0x00AABBCC)).unwrap();
    assert_eq!(&p[4..7], &[0xCC, 0xBB, 0xAA], "little-endian byte order");
    assert_eq!(p[7], 0x04, "adjacent byte untouched");

    let m = single_field_module(
        "big",
        "{ id: 'a', label: 'A', offset: 0, size: 3, type: 'uint' }",
    );
    let mut p = payload();
    write_field(&mut p, &m, 0, "a", &FieldValue::Uint(0x00AABBCC)).unwrap();
    assert_eq!(&p[4..7], &[0xAA, 0xBB, 0xCC], "big-endian byte order");
}

#[test]
fn uint_write_rejects_values_above_the_stored_range() {
    for (size, max) in [(1u64, 255u64), (2, 65535), (3, 16777215), (4, 4294967295)] {
        let m = single_field_module(
            "little",
            &format!("{{ id: 'a', label: 'A', offset: 0, size: {size}, type: 'uint' }}"),
        );
        let mut p = payload();
        assert!(write_field(&mut p, &m, 0, "a", &FieldValue::Uint(max)).is_ok());
        let err = write_field(&mut p, &m, 0, "a", &FieldValue::Uint(max + 1));
        assert_eq!(
            err,
            Err(RuntimeError::ValueOutOfRange {
                field_id: "a".into(),
                value: i128::from(max) + 1,
                min: 0,
                max: i128::from(max),
            }),
            "size {size}"
        );
    }
}

#[test]
fn int_write_round_trips_boundaries_and_rejects_outside() {
    for (size, min, max) in [
        (1u64, i64::from(i8::MIN), i64::from(i8::MAX)),
        (2, i64::from(i16::MIN), i64::from(i16::MAX)),
        (4, i64::from(i32::MIN), i64::from(i32::MAX)),
    ] {
        let m = single_field_module(
            "little",
            &format!("{{ id: 'a', label: 'A', offset: 0, size: {size}, type: 'int' }}"),
        );
        let mut p = payload();
        for value in [min, -1, 0, max] {
            write_field(&mut p, &m, 0, "a", &FieldValue::Int(value)).unwrap();
            assert_eq!(
                read(&m, &p, 0),
                FieldValue::Int(value),
                "size {size} value {value}"
            );
        }
        assert!(write_field(&mut p, &m, 0, "a", &FieldValue::Int(min - 1)).is_err());
        assert!(write_field(&mut p, &m, 0, "a", &FieldValue::Int(max + 1)).is_err());
    }
}

#[test]
fn int_write_stores_twos_complement_bytes() {
    let m = single_field_module(
        "little",
        "{ id: 'a', label: 'A', offset: 0, size: 2, type: 'int' }",
    );
    let mut p = payload();
    write_field(&mut p, &m, 0, "a", &FieldValue::Int(-2)).unwrap();
    assert_eq!(&p[4..6], &[0xFE, 0xFF]);
}

// --- writing bytes and text -------------------------------------------------------------------

#[test]
fn bytes_write_requires_the_exact_declared_length() {
    let m = single_field_module(
        "little",
        "{ id: 'a', label: 'A', offset: 0, size: 4, type: 'bytes' }",
    );
    let mut p = payload();
    write_field(&mut p, &m, 0, "a", &FieldValue::Bytes(vec![9, 8, 7, 6])).unwrap();
    assert_eq!(&p[4..8], &[9, 8, 7, 6]);

    for wrong in [vec![], vec![1, 2, 3], vec![1, 2, 3, 4, 5]] {
        let actual = wrong.len();
        assert_eq!(
            write_field(&mut p, &m, 0, "a", &FieldValue::Bytes(wrong)),
            Err(RuntimeError::InvalidBytesLength {
                field_id: "a".into(),
                expected: 4,
                actual,
            })
        );
    }
}

#[test]
fn text_write_pads_with_nul_to_the_field_size() {
    let m = text_module();
    let mut p = *b"XXXXXXXX";
    write_field(&mut p, &m, 0, "t", &FieldValue::Text("AB".into())).unwrap();
    assert_eq!(&p, b"AB\0\0\0\0\0\0");
}

#[test]
fn text_write_accepts_a_full_length_string() {
    let m = text_module();
    let mut p = [0u8; 8];
    write_field(&mut p, &m, 0, "t", &FieldValue::Text("ABCDEFGH".into())).unwrap();
    assert_eq!(&p, b"ABCDEFGH");
}

#[test]
fn text_write_rejects_over_length_strings() {
    let m = text_module();
    let mut p = [0u8; 8];
    let err = write_field(&mut p, &m, 0, "t", &FieldValue::Text("ABCDEFGHI".into()));
    assert!(matches!(err, Err(RuntimeError::InvalidText { .. })));
}

#[test]
fn text_write_rejects_non_printable_ascii() {
    let m = text_module();
    let mut p = [0u8; 8];
    for bad in ["é", "tab\there", "new\nline", "nul\0", "\u{FFFD}"] {
        let err = write_field(&mut p, &m, 0, "t", &FieldValue::Text(bad.into()));
        assert!(
            matches!(err, Err(RuntimeError::InvalidText { .. })),
            "{bad:?} should be rejected"
        );
    }
}

/// The spec's no-round-trip rule: a lossy read result must not be writable.
#[test]
fn lossy_text_reads_cannot_be_written_back() {
    let m = text_module();
    let p = [0x41, 0x80, 0x42, 0x00, 0x00, 0x00, 0x00, 0x00];
    let FieldValue::Text(displayed) = read(&m, &p, 0) else {
        panic!("expected text");
    };
    assert_eq!(displayed, "A\u{FFFD}B");

    let mut copy = p;
    let err = write_field(&mut copy, &m, 0, "t", &FieldValue::Text(displayed));
    assert!(matches!(err, Err(RuntimeError::InvalidText { .. })));
    assert_eq!(copy, p, "payload must be untouched after a rejected write");
}

// --- write errors ------------------------------------------------------------------------------

#[test]
fn writes_reject_mismatched_value_variants() {
    let m = module(
        "little",
        4,
        "{ count: 2, size: 8 }",
        "{ id: 'u', label: 'U', offset: 0, size: 2, type: 'uint' },
         { id: 'i', label: 'I', offset: 2, size: 2, type: 'int' },
         { id: 'b', label: 'B', offset: 4, size: 2, type: 'bytes' },
         { id: 't', label: 'T', offset: 6, size: 2, type: 'text' }",
    );
    let mut p = payload();

    let cases: [(&str, FieldValue, &str); 4] = [
        ("u", FieldValue::Int(1), "uint"),
        ("i", FieldValue::Uint(1), "int"),
        ("b", FieldValue::Text("ab".into()), "bytes"),
        ("t", FieldValue::Bytes(vec![1, 2]), "text"),
    ];
    for (id, value, expected) in cases {
        assert_eq!(
            write_field(&mut p, &m, 0, id, &value),
            Err(RuntimeError::TypeMismatch {
                field_id: id.into(),
                expected,
            })
        );
    }
}

#[test]
fn writes_to_sections_unknown_fields_and_bad_indexes_are_rejected() {
    let m = module(
        "little",
        4,
        "{ count: 2, size: 8 }",
        "{ id: 'a', label: 'A', offset: 0, size: 1, type: 'uint' },
         { id: 's', label: 'S', type: 'section' }",
    );
    let mut p = payload();

    assert_eq!(
        write_field(&mut p, &m, 0, "s", &FieldValue::Uint(1)),
        Err(RuntimeError::SectionHasNoValue {
            field_id: "s".into()
        })
    );
    assert_eq!(
        write_field(&mut p, &m, 0, "missing", &FieldValue::Uint(1)),
        Err(RuntimeError::FieldNotFound {
            field_id: "missing".into()
        })
    );
    assert_eq!(
        write_field(&mut p, &m, 5, "a", &FieldValue::Uint(1)),
        Err(RuntimeError::IndexOutOfRange { index: 5, count: 2 })
    );
}

// --- overlapping views ---------------------------------------------------------------------------

#[test]
fn overlapping_fields_see_each_others_writes() {
    let m = module(
        "little",
        4,
        "{ count: 2, size: 8 }",
        "{ id: 'narrow', label: 'N', offset: 0, size: 1, type: 'uint' },
         { id: 'wide', label: 'W', offset: 0, size: 2, type: 'uint' }",
    );
    let mut p = payload();

    write_field(&mut p, &m, 0, "wide", &FieldValue::Uint(0xBEEF)).unwrap();
    assert_eq!(
        read_field(&p, &m, 0, &m.fields[0]).unwrap(),
        FieldValue::Uint(0xEF),
        "narrow view reflects the low byte of the wide write"
    );

    write_field(&mut p, &m, 0, "narrow", &FieldValue::Uint(0x12)).unwrap();
    assert_eq!(
        read_field(&p, &m, 0, &m.fields[1]).unwrap(),
        FieldValue::Uint(0xBE12),
        "wide view reflects the narrow write; last write wins"
    );
}

// --- dynamic count end to end ----------------------------------------------------------------------

#[test]
fn battle_unit_style_dynamic_count_reads_and_writes() {
    // Header count at 0x20 + 0x04 (LE u32), table at 0x30, 4-byte entries.
    let m = module(
        "little",
        0x30,
        "{ count_from: { base_offset: 0x20, offset: 0x04, size: 4, type: 'uint' }, size: 4 }",
        "{ id: 'a', label: 'A', offset: 0, size: 4, type: 'uint' }",
    );

    let mut p = vec![0u8; 0x30 + 8]; // room for 2 entries
    p[0x24] = 2; // count = 2
    p[0x30..0x34].copy_from_slice(&[0x01, 0, 0, 0]);
    p[0x34..0x38].copy_from_slice(&[0x02, 0, 0, 0]);

    assert_eq!(resolve_count(&p, &m), Ok(2));
    assert_eq!(read(&m, &p, 1), FieldValue::Uint(2));
    assert_eq!(
        read_field(&p, &m, 2, &m.fields[0]),
        Err(RuntimeError::IndexOutOfRange { index: 2, count: 2 })
    );

    write_field(&mut p, &m, 1, "a", &FieldValue::Uint(0xDEAD)).unwrap();
    assert_eq!(read(&m, &p, 1), FieldValue::Uint(0xDEAD));

    // Shrinking the stored count makes index 1 invalid.
    p[0x24] = 1;
    assert_eq!(
        write_field(&mut p, &m, 1, "a", &FieldValue::Uint(1)),
        Err(RuntimeError::IndexOutOfRange { index: 1, count: 1 })
    );
}
