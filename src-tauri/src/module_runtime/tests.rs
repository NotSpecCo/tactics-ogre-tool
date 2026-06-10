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

/// A payload with an `xlce` block header at offset 0 (so the table base is
/// 0x10) followed by `entries` entries of `entry_size` filler bytes each.
/// Header integers are encoded with `big = false` for little-endian.
fn header_payload(count: u32, entry_size: u32, entries: usize, big: bool) -> Vec<u8> {
    let mut p = Vec::new();
    p.extend_from_slice(b"xlce");
    for value in [count, 0x10, entry_size] {
        if big {
            p.extend_from_slice(&value.to_be_bytes());
        } else {
            p.extend_from_slice(&value.to_le_bytes());
        }
    }
    p.extend_from_slice(&vec![0x55u8; entries * entry_size as usize]);
    p
}

const HEADER_ENTRY: &str = "{ header: true, size: 1 }";
const FIELD_U8: &str = "{ id: 'a', label: 'A', offset: 0, size: 1, type: 'uint' }";

#[test]
fn header_count_reads_little_and_big_endian() {
    let m = module("little", 0x10, HEADER_ENTRY, FIELD_U8);
    assert_eq!(resolve_count(&header_payload(3, 1, 3, false), &m), Ok(3));

    let m = module("big", 0x10, HEADER_ENTRY, FIELD_U8);
    assert_eq!(resolve_count(&header_payload(3, 1, 3, true), &m), Ok(3));
}

#[test]
fn header_count_zero_resolves_to_an_empty_table() {
    let m = module("little", 0x10, HEADER_ENTRY, FIELD_U8);
    assert_eq!(resolve_count(&header_payload(0, 1, 0, false), &m), Ok(0));
}

#[test]
fn header_region_outside_payload_is_rejected() {
    let m = module("little", 0x10, HEADER_ENTRY, FIELD_U8);
    // Payload too short to hold the 16-byte header.
    assert_eq!(
        resolve_count(&[0u8; 8], &m),
        Err(RuntimeError::HeaderOutOfBounds {
            base_offset: 0x10,
            payload_len: 8
        })
    );
}

#[test]
fn header_base_offset_below_the_header_size_is_rejected() {
    // The loader's validation forbids this; the runtime must still guard
    // against a hand-built module rather than underflow.
    let m = module("little", 4, HEADER_ENTRY, FIELD_U8);
    assert_eq!(
        resolve_count(&header_payload(1, 1, 1, false), &m),
        Err(RuntimeError::HeaderOutOfBounds {
            base_offset: 4,
            payload_len: 17
        })
    );
}

#[test]
fn bad_header_magic_is_rejected() {
    let m = module("little", 0x10, HEADER_ENTRY, FIELD_U8);
    let mut p = header_payload(1, 1, 1, false);
    p[0..4].copy_from_slice(b"XLCE");
    assert_eq!(
        resolve_count(&p, &m),
        Err(RuntimeError::BadHeaderMagic { found: *b"XLCE" })
    );
}

#[test]
fn bad_header_data_offset_is_rejected() {
    let m = module("little", 0x10, HEADER_ENTRY, FIELD_U8);
    let mut p = header_payload(1, 1, 1, false);
    p[8] = 0x20; // data start offset 0x20 instead of 0x10
    assert_eq!(
        resolve_count(&p, &m),
        Err(RuntimeError::BadHeaderDataOffset { found: 0x20 })
    );
}

#[test]
fn header_entry_size_mismatch_is_rejected() {
    // Header declares 4-byte entries; the module's field layout is 1-byte.
    let m = module("little", 0x10, HEADER_ENTRY, FIELD_U8);
    assert_eq!(
        resolve_count(&header_payload(1, 4, 1, false), &m),
        Err(RuntimeError::HeaderEntrySizeMismatch {
            header_size: 4,
            entry_size: 1
        })
    );
}

#[test]
fn header_resolved_table_outside_payload_is_rejected() {
    let m = module("little", 0x10, HEADER_ENTRY, FIELD_U8);
    // Header count says 200 one-byte entries; payload holds 3.
    let p = header_payload(200, 1, 3, false);
    assert_eq!(
        resolve_count(&p, &m),
        Err(RuntimeError::TableOutOfBounds {
            required: 0x10 + 200,
            payload_len: p.len(),
        })
    );
}

// --- count_divergence -----------------------------------------------------------------

#[test]
fn count_divergence_only_fires_for_differing_expected_counts() {
    let fixed = module("little", 4, "{ count: 2, size: 8 }", FIELD_U8);
    assert_eq!(count_divergence(&fixed, 5), None);

    let no_expected = module("little", 0x10, HEADER_ENTRY, FIELD_U8);
    assert_eq!(count_divergence(&no_expected, 5), None);

    let expected = module(
        "little",
        0x10,
        "{ header: true, count: 3, size: 1 }",
        FIELD_U8,
    );
    assert_eq!(count_divergence(&expected, 3), None);
    assert_eq!(
        count_divergence(&expected, 5),
        Some(CountDivergence {
            expected: 3,
            actual: 5
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

// --- header-driven count end to end ----------------------------------------------------------------------

#[test]
fn battle_unit_style_header_count_reads_and_writes() {
    // Block header at 0x20 (so the table base is 0x30), 4-byte entries —
    // the BattleUnit layout.
    let m = module(
        "little",
        0x30,
        "{ header: true, size: 4 }",
        "{ id: 'a', label: 'A', offset: 0, size: 4, type: 'uint' }",
    );

    let mut p = vec![0u8; 0x30 + 8]; // room for 2 entries
    p[0x20..0x24].copy_from_slice(b"xlce");
    p[0x24] = 2; // count = 2
    p[0x28] = 0x10; // data start offset
    p[0x2c] = 4; // entry size
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

    // Shrinking the stored header count makes index 1 invalid.
    p[0x24] = 1;
    assert_eq!(
        write_field(&mut p, &m, 1, "a", &FieldValue::Uint(1)),
        Err(RuntimeError::IndexOutOfRange { index: 1, count: 1 })
    );
}

// --- size helper guards ------------------------------------------------------------
//
// Sizes are normally guaranteed by module_spec::parse; these helpers must
// panic (not silently degrade) when a hand-built ModuleFile bypasses it.

#[test]
fn integer_helpers_accept_every_valid_size() {
    assert_eq!(max_unsigned(1), 0xff);
    assert_eq!(max_unsigned(2), 0xffff);
    assert_eq!(max_unsigned(3), 0xff_ffff);
    assert_eq!(max_unsigned(4), 0xffff_ffff);

    assert_eq!(signed_range(1), (i64::from(i8::MIN), i64::from(i8::MAX)));
    assert_eq!(signed_range(2), (i64::from(i16::MIN), i64::from(i16::MAX)));
    assert_eq!(signed_range(4), (i64::from(i32::MIN), i64::from(i32::MAX)));

    assert_eq!(sign_extend(0xff, 1), -1);
    assert_eq!(sign_extend(0xffff, 2), -1);
    assert_eq!(sign_extend(0xffff_ffff, 4), -1);
}

#[test]
#[should_panic(expected = "invalid integer field size 0")]
fn max_unsigned_panics_on_zero_size() {
    max_unsigned(0);
}

#[test]
#[should_panic(expected = "invalid integer field size 5")]
fn max_unsigned_panics_on_oversized() {
    max_unsigned(5);
}

#[test]
#[should_panic(expected = "invalid int field size 3")]
fn signed_range_panics_on_three_byte_int() {
    signed_range(3);
}

#[test]
#[should_panic(expected = "invalid int field size 0")]
fn signed_range_panics_on_zero_size() {
    signed_range(0);
}

#[test]
#[should_panic(expected = "invalid int field size 3")]
fn sign_extend_panics_on_three_byte_int() {
    sign_extend(0, 3);
}

#[test]
#[should_panic(expected = "invalid int field size 8")]
fn sign_extend_panics_on_oversized() {
    sign_extend(0, 8);
}
