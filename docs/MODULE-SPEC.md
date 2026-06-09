# Module Spec (JSON5)

Modules define how to read and edit structured binary tables inside selected game `.dat` files. Each module is a `.json5` file loaded from the filesystem at runtime.

Module and sidecar files parse with a standard JSON5 parser. Strings are always quoted, so labels like `No` or `Tactics Ogre: Reborn` need no special handling. JSON5 comments (`//` and `/* */`) are permitted and ignored by loaders; tools that rewrite files are not required to preserve them. Text that must survive belongs in `notes`. All property names and module file names are lowercase `snake_case`.

This document defines version `1` of the module format. Version `1` is scoped to the Tactics Ogre modules in `reference_files/nightmare_modules_new`.

## Reference Files

Use these when researching conversion behavior. The new Tactics Ogre module set is authoritative for conversion; older modules and Nightmare's bundled examples are historical only and should not shape version `1`.

1. `reference_files/nightmare` - source code for original Nightmare app
2. `reference_files/nightmare/doc` - Nightmare documentation
3. `reference_files/nightmare/doc/Original Nightmare Format.txt` - original module file format
4. `reference_files/nightmare/doc/New Nightmare Format.txt` - new module file format
5. `reference_files/nightmare_modules_new` - new Nightmare modules. This set is the source of truth for field layouts.

## Directory Structure

```text
modules/
|-- battle_armament.json5
|-- battle_hit_rate.json5
|-- menu_shop.json5
|-- options/                // dropdown option files
|   |-- no_yes.json5
|   |-- item_type.json5
|   `-- ...
`-- entries/                // entry label files
    |-- class.json5
    |-- skill.json5
    `-- ...
```

## Path Rules

Module `files` are game-root-relative POSIX paths to `.dat` files. They must not begin with `/`.

```json5
files: [
    'battle/battle_data_release.dat',
    'battle/entry/entry_unit_*.dat'
],
```

`files` entries may be literal paths or glob patterns using `*` and `?`, with standard POSIX glob semantics: `*` and `?` match within a single path segment and never match `/`. Directory targets are not part of version `1`; a directory path is invalid. A glob matches files only, not directories. Target matching is case-insensitive, so `battle/entry/entry_unit_*.dat` matches both `entry_unit_0001.dat` and `ENTRY_UNIT_0001.dat`.

The user selects a `.dat` file. The app decrypts and unpacks that file in the background, keeps the resulting payload in memory, and applies module offsets to that in-memory payload. Each supported `.dat` must resolve to exactly one editable payload. If the selected `.dat` resolves to zero payloads, multiple editable payloads, or an unpacking shape the app cannot map to one payload, the app must throw an error and leave UI presentation of that error to the caller. `base_offset`, field `offset`, and `entry.count_from` offsets are never relative to the encrypted `.dat` container or the intermediate zip wrapper.

Sidecar paths such as `entry.labels_file` and `options_file` are relative to the module file's directory and must stay within the module tree after normalization.

## Module File Format

```json5
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
```

## Module Keys

| Key              | Required | Description                                                                     |
| ---------------- | -------- | ------------------------------------------------------------------------------- |
| `schema_version` | yes      | Must be `1`.                                                                    |
| `id`             | yes      | Stable module identifier. Use lowercase ASCII letters, digits, and underscores. |
| `label`          | yes      | Human-readable module name.                                                     |
| `notes`          | no       | Help text shown to the user in the UI.                                          |
| `source`         | no       | Provenance metadata. Ignored by binary readers and writers.                     |
| `files`          | yes      | Game-root-relative file paths or glob patterns this module applies to.          |
| `base_offset`    | yes      | Byte offset of the first table entry in each matched target file.               |
| `endian`         | no       | `little` or `big`. Defaults to `little`.                                        |
| `entry`          | yes      | Entry count, size, and optional label sidecar.                                  |
| `fields`         | yes      | Ordered field and section definitions.                                          |

`id` values are persistence keys. They must not be generated from `label` at load time.

`notes` is optional help text shown to the user in the UI. Anything that has a `label` may also have `notes`: modules, fields (including sections), options, and entries.

## Entry Definition

Most tables have a fixed entry count:

```json5
entry: {
    count: 256,
    size: 208,
    labels_file: 'entries/class.json5'
},
```

Tables whose count is stored in the target file use `count_from`:

```json5
entry: {
    count_from: {
        base_offset: 0x20,
        offset: 0x04,
        size: 4,
        type: 'uint'
    },
    size: 0xc4,
    labels_file: null
},
```

`count` and `count_from` are mutually exclusive.

`labels_file` is optional. Omitting it and setting it to `null` are equivalent: the table has no entry labels.

`count_from` requires `offset`, `size`, and `type`; `base_offset` and `endian` are optional.

The count is always read from the current matched target file. `count_from.base_offset` defaults to `0`. The final count address is `count_from.base_offset + count_from.offset`.

`count_from.type` must be `uint` in version `1`. `count_from.size` must be `1`, `2`, or `4`, and uses the module's `endian` unless `count_from.endian` is provided.

## Field Types

| Type       | Stored value             | Required keys                                           | Notes                                                         |
| ---------- | ------------------------ | ------------------------------------------------------- | ------------------------------------------------------------- |
| `uint`     | Unsigned integer         | `id`, `label`, `offset`, `size`, `type`                 | `size` must be `1`, `2`, `3`, or `4`.                         |
| `int`      | Signed integer           | `id`, `label`, `offset`, `size`, `type`                 | `size` must be `1`, `2`, or `4`. Always displayed as decimal. |
| `bytes`    | Raw bytes                | `id`, `label`, `offset`, `size`, `type`                 | `size` may be any positive integer. Always displayed as hex.  |
| `text`     | Fixed-length ASCII bytes | `id`, `label`, `offset`, `size`, `type`                 | `size` may be any positive integer.                           |
| `dropdown` | Unsigned integer         | `id`, `label`, `offset`, `size`, `type`, `options_file` | `size` must be `1`, `2`, `3`, or `4`.                         |
| `section`  | none                     | `id`, `label`, `type`                                   | Visual heading only. No offset, size, or stored value.        |

Common optional field keys:

| Key       | Applies to                | Description                                |
| --------- | ------------------------- | ------------------------------------------ |
| `display` | `uint`, `dropdown`        | `decimal` or `hex`. Defaults to `decimal`. |
| `endian`  | `uint`, `int`, `dropdown` | Overrides module `endian`.                 |
| `notes`   | all fields                | Help text shown to the user in the UI.     |

`display` affects only presentation and parsing of user input. It does not change the stored bytes.

Multi-byte integer fields use `endian` to determine byte order. For 3-byte `uint` and `dropdown` fields, little-endian stores the least significant byte first and big-endian stores the most significant byte first.

## Numeric Rules

Integers are native JSON5 numeric literals, written as unsigned decimal (`256`) or unsigned hexadecimal (`0x0100`) values. Hexadecimal digits are case-insensitive.

Any other form is invalid where the schema expects an integer: leading `+` or `-` signs, fractional parts, exponents, `Infinity`, `NaN`, booleans, nulls, arrays, objects, and strings containing numbers such as `"256"` or `"0x0100"`. After parsing, the normal validation rules still apply, such as non-negative, positive, and stored-range checks.

Stored integer ranges are determined by `type` and `size`:

| Type                | Size | Range                         |
| ------------------- | ---- | ----------------------------- |
| `uint` / `dropdown` | `1`  | `0` to `255`                  |
| `uint` / `dropdown` | `2`  | `0` to `65535`                |
| `uint` / `dropdown` | `3`  | `0` to `16777215`             |
| `uint` / `dropdown` | `4`  | `0` to `4294967295`           |
| `int`               | `1`  | `-128` to `127`               |
| `int`               | `2`  | `-32768` to `32767`           |
| `int`               | `4`  | `-2147483648` to `2147483647` |

Writers must reject values outside the field's range.

## Text Rules

Version `1` supports fixed-length ASCII text fields only.

Readers decode bytes up to the first `0x00` byte, or the full field size if no terminator is present. Bytes outside printable ASCII (`0x20` to `0x7e`) decode to the `U+FFFD` replacement character. This decoding is lossy and display-only: the app must never write a text field by re-encoding its displayed string, only from explicit user input.

Writers accept printable ASCII (`0x20` to `0x7e`) only. They reject any other character, reject strings longer than `size`, and pad the remaining bytes with `0x00`.

Character mapping tables are not part of version `1`.

> **Implementation note:** when building the JSON5 module reader, cover the lossy `U+FFFD` read path and the no-round-trip rule in tests. The reference set contains a single `TEXT` field, so these paths are easy to miss.

## Duplicate and Overlapping Storage

Duplicate offsets are allowed. Nightmare modules intentionally expose the same stored value through multiple interpretations, such as a raw numeric field plus one or more dropdowns.

Each field is an independent view over its declared storage. Editing any field writes that field's bytes. Other fields that read the same bytes must be refreshed from storage after the write. There is no field priority; the last successful write wins.

Overlap rules:

1. Identical storage ranges are allowed.
2. Partially overlapping byte ranges are allowed but produce a validation warning, since they are usually offset mistakes.
3. A `section` never participates in overlap checks.

## Bit-Level Fields

Bit-level fields are not part of version `1`.

The authoritative `reference_files/nightmare_modules_new` set does not use Nightmare's lowercase `b` bit-suffixed offsets or sizes. Future versions can add bit support if broader Nightmare compatibility becomes a goal.

## Options Files

Used by `dropdown` fields. Each option has an integer value, label, and optional notes. The file is a top-level JSON5 array.

```json5
[
    { value: 0x00, label: 'None' },
    { value: 0x01, label: 'Sword', notes: 'Standard one-handed blade' },
    { value: 0x02, label: 'Axe' },
    { value: 0x03, label: 'Spear' }
]
```

Option files are ordered arrays, not maps. The loader must preserve file order.

Option semantics:

1. Values may be decimal or hex.
2. Values need not be contiguous.
3. Duplicate values are allowed as aliases. When reading a stored value with multiple labels, the first matching option is the canonical display label.
4. Empty labels are allowed but should produce a validation warning. UIs should fall back to the numeric value when a label is empty.
5. Missing values do not compact or renumber anything.
6. Values must fit in the dropdown field size that references them.
7. A stored value with no matching option is valid, not an error. The UI displays the raw numeric value using the field's `display` format, and the stored bytes keep that value until the user explicitly selects an option or enters a new value. Loading and saving an entry must never coerce an unmatched value to any option.

## Entry Files

Used by `entry.labels_file`. Entry files use the same item shape as options files:

```json5
[
    { value: 0x00, label: 'Broad Sword' },
    { value: 0x01, label: 'Short Sword' },
    { value: 0x02, label: 'Battle Axe', notes: 'Two-handed axe with high damage' }
]
```

The value is the entry index. Entry files are ordered arrays, but lookup is by `value`.

Entry semantics:

1. Missing entry indexes are allowed.
2. Missing entries do not compact or renumber later entries.
3. If an entry index has no label, the UI displays the numeric index.
4. Entries marked `<not used>`, `<deleted>`, or `<<Nothing>>` should normally be kept during automated conversion because they preserve source intent. They may be omitted manually only if the numeric index semantics remain unchanged.
5. Entry values at or above the resolved entry count are allowed but never displayed; the UI ignores them. This supports sharing one entry file between modules with different counts, such as `entries/class.json5` used by both `battle_class` (count 256) and `menu_classmark` (count 80).

## Validation Rules

A module set is valid only if all modules and sidecars pass these checks.

Module validation:

1. `schema_version` is `1`.
2. Module `id` is unique across the loaded module set.
3. Module `id` and field `id` values match `^[a-z][a-z0-9_]*$`.
4. `files` is non-empty, and every item is a relative POSIX path or glob.
5. `files` entries do not contain `..`, do not begin with `/`, and do not resolve outside the game root.
6. `base_offset` is a non-negative integer.
7. `endian` is `little` or `big`.
8. `entry.size` is a positive integer.
9. Exactly one of `entry.count` or `entry.count_from` is present.
10. Fixed `entry.count` is a positive integer.
11. `entry.count_from`, when present, has a non-negative `offset`; a non-negative `base_offset` when present; a `size` of `1`, `2`, or `4`; a `type` of `uint`; and an `endian` of `little` or `big` when present.
12. `fields` is non-empty.
13. Field IDs are unique within a module.

Field validation:

1. Non-section fields have `offset` and `size`.
2. Section fields do not have `offset`, `size`, `options_file`, `display`, or `endian`.
3. Non-section `offset` is a non-negative integer.
4. Non-section `size` is a positive integer.
5. Each non-section field satisfies `offset + size <= entry.size`.
6. `uint` and `dropdown` sizes are `1`, `2`, `3`, or `4`.
7. `int` sizes are `1`, `2`, or `4`.
8. `bytes` and `text` sizes may be any positive integer.
9. Only `uint` and `dropdown` fields use `display`.
10. `dropdown` has an `options_file`.
11. Non-dropdown fields do not have `options_file`.
12. Sidecar paths exist and are valid relative paths.
13. Partially overlapping byte ranges are warnings, not errors.

Sidecar validation:

1. Sidecar files parse as top-level JSON5 arrays.
2. Every item has `value` and `label`.
3. Values are non-negative integers.
4. Labels are strings.
5. Duplicate values are warnings, not errors.
6. Empty labels are warnings, not errors.
7. Option values fit every dropdown size that references the option file.
8. Entry values at or above a referencing module's fixed entry count are warnings, not errors.

Runtime payload validation:

1. `base_offset + entry.count * entry.size` must be within the decrypted/unpacked in-memory payload length.
2. For dynamic counts, the full `count_from` integer range must be within the matched target payload length before the count is read.
3. After resolving a dynamic count, `base_offset + resolved_count * entry.size` must be within the matched target payload length.
4. Every stored field range for every resolved entry, `base_offset + entry_index * entry.size + field.offset` through `size`, must be within the matched target payload length.

If any runtime payload validation check fails, the reader or writer must throw an error and leave UI presentation of that error to the caller.

## Converting Nightmare Modules

The existing `.nmm` modules in `reference_files/nightmare_modules_new/` are the source of truth for field layouts. Each `.nmm` file, along with the three shared reference directories (`_record`, `_list`, `_name`), converts to this JSON5 format.

### Nightmare Directory Mapping

| Nightmare            | Ours                   | Purpose                                        |
| -------------------- | ---------------------- | ---------------------------------------------- |
| `_record/*.txt`      | `entries/*.json5`      | Row labels for entry selectors                 |
| `_list/*.txt`        | `options/*.json5`      | Field dropdowns                                |
| `_name/*.txt`        | `options/name_*.json5` | Field dropdowns that reference game string IDs |
| `battle/*.nmm`       | `modules/*.json5`      | Module definitions                             |
| `battle/entry/*.nmm` | `modules/*.json5`      | Battle entry module definitions                |
| `menu/*.nmm`         | `modules/*.json5`      | Module definitions                             |

`_list` and `_name` both become `options/` files. `_name` outputs use a `name_` filename prefix to avoid collisions with `_list` files that share the same basename, such as `_list/Class.txt` and `_name/Class.txt`. `_record` is the only source that becomes `entries/`.

Converted file names are the normalized source basename (see Name Normalization), plus the `name_` prefix for `_name` sources. `_list/ItemType.txt` becomes `options/item_type.json5`; `_record/BattleUnitHeader.txt` becomes `entries/battle_unit_header.json5`; `_name/UnitName.txt` becomes `options/name_unit_name.json5`.

### Name Normalization

Converted identifiers and file names use lowercase `snake_case`, produced by one normalization:

1. Insert an underscore at each camel case word boundary: between a lowercase letter and an uppercase letter, and between an uppercase letter and an uppercase letter followed by a lowercase letter.
2. Lowercase all ASCII letters.
3. Replace each run of characters outside `a-z` and `0-9` with a single underscore.
4. Trim leading and trailing underscores.

`ItemType` becomes `item_type`. `BattleUnitHeader` becomes `battle_unit_header`. `Skill: <not used> 0006` becomes `skill_not_used_0006`. `? Flag ?` becomes `flag`. `??` becomes the empty string.

### .nmm Parsing Rules

All Nightmare source files (`.nmm`, `_list`, `_name`, `_record`) are split into lines on any newline convention: `\n`, `\r\n`, or `\r`, including a mix within one file. The reference set uses CRLF throughout, but converters must not depend on that, since module developers may edit these files on Windows, macOS, or Linux. After splitting, trailing whitespace is trimmed from each line so no label or path carries a stray `\r`.

Nightmare allows blank lines and `#`-prefixed comments almost anywhere. The converter reads `.nmm` files as logical lines by discarding blank lines and comment lines before parsing headers or fields.

The first three comment lines in each module, followed by one blank line, are Nightmare-specific file verification metadata:

```text
#181ADE14          # 32-bit checksum of the target file
#0x003BA490        # address of an ID string in the target file
#xlce              # ASCII ID string expected at that address
                   # BASEPOINTER flag, blank in these modules
```

This metadata is not represented directly in the module file. Its file-verification role is replaced by the module's explicit `files` list. The converter skips the first three comment lines and never converts them to `notes`. It may preserve them under `source` if useful.

All other comments convert by position:

1. Prose comments after the metadata block and before the `.nmm` header convert to module `notes`.
2. Comment lines inside a field's five logical lines convert to that field's `notes`. In this module set, these always appear immediately after the field label.
3. Comment lines between field blocks are not converted automatically because they usually represent disabled fields (fully commented-out five-line blocks) or scratch notes.

When multiple comment lines convert to the same `notes` value, strip each line's leading `#`, trim surrounding whitespace, and join the lines with newlines in file order.

### .nmm Header Conversion

After comments and blank lines are removed, the `.nmm` header contains 7 logical lines:

```text
1                              # number of files, ignored
Reborn ==> ... ==> Class       # Nightmare title
0x003BA4A0                     # base_offset
0x00000100                     # entry_count
0x000000D0                     # entry_size
../_record/Class.txt           # entry labels file, or NULL
NULL                           # character table file, ignored in v1
```

Header conversion rules:

1. `id` is the source path without extension, with each path segment normalized (see Name Normalization) and segments joined by underscores. `battle/Class.nmm` becomes `battle_class`; `battle/entry/BattleUnit.nmm` becomes `battle_entry_battle_unit`.
2. `label` is the final segment after the last `==>`, trimmed.
3. `source.format` is `nightmare`.
4. `source.path` is the source path relative to `reference_files/nightmare_modules_new`.
5. `source.title` is the full Nightmare title.
6. Numeric header values may be decimal or hex.
7. `entry.labels_file` is the converted `_record` file path, or `null` for `NULL`.
8. The Nightmare character table path is ignored in version `1`.

Target file conversion rules:

| Nightmare title segment                   | Module `files`                   |
| ----------------------------------------- | -------------------------------- |
| `battle_data_release / pack`              | `battle/battle_data_release.dat` |
| `menu_data / pack`                        | `menu/menu_data.dat`             |
| `battle / entry / entry_unit_#### / pack` | `battle/entry/entry_unit_*.dat`  |

### Dynamic Battle Unit Counts

`battle/entry/BattleUnitHeader.nmm` converts as a normal fixed-count module:

```json5
files: ['battle/entry/entry_unit_*.dat'],
base_offset: 0x20,
entry: {
    count: 1,
    size: 0x10,
    labels_file: 'entries/battle_unit_header.json5'
},
```

`battle/entry/BattleUnit.nmm` uses the header's record count field:

```json5
files: ['battle/entry/entry_unit_*.dat'],
base_offset: 0x30,
entry: {
    count_from: {
        base_offset: 0x20,
        offset: 0x04,
        size: 4,
        type: 'uint'
    },
    size: 0xc4,
    labels_file: null
},
```

### .nmm Field Type Codes

Each Nightmare field is 5 logical lines: label, offset, size in bytes, type code, and options file or `NULL`.

| Code   | Meaning                            | Module type | Module display |
| ------ | ---------------------------------- | ----------- | -------------- |
| `NEDU` | Normal edit, decimal, unsigned     | `uint`      | `decimal`      |
| `NEDS` | Normal edit, decimal, signed       | `int`       | n/a            |
| `NEHU` | Normal edit, hex, unsigned         | `uint`      | `hex`          |
| `NDDU` | Normal dropdown, decimal, unsigned | `dropdown`  | `decimal`      |
| `NDHU` | Normal dropdown, hex, unsigned     | `dropdown`  | `hex`          |
| `HEXA` | Raw hex byte dump                  | `bytes`     | n/a            |
| `TEXT` | Fixed-length text bytes            | `text`      | n/a            |

`NEHU` fields with size `3` convert to `uint` fields with `size: 3` and `display: 'hex'`.

Nightmare sidecar paths are resolved relative to the source `.nmm` file before they are mapped to module paths. For example, both `../_list/Class.txt` from `battle/Class.nmm` and `../../_list/Class.txt` from `battle/entry/BattleUnit.nmm` resolve to the same `options/class.json5` output file.

Fields whose options file is `_list/separator.txt` convert to `section` entries. Keep the Nightmare label as the section label and discard the original offset, size, type code, and options file.

Partially overlapping byte ranges in the source convert as-is and surface as validation warnings. The reference set contains two: in `battle/Armament.nmm` and `battle/Class.nmm`, a 1-byte set-reference dropdown overlaps a 2-byte padding field at offset `0x00`.

Generated field IDs are normalized labels (see Name Normalization). If the normalized label is empty or does not start with a letter, the fallback ID is `field_0x<offset>` for stored fields, written in lowercase hex padded to at least two digits, or `section_<ordinal>` for sections, where the ordinal is the section's 1-based position among the module's sections. `? 23 Flag ?` at offset `0x23` becomes `field_0x23`.

Deduplication runs after all IDs are generated, including fallbacks: in field order, the second and later occurrences of an ID get `_2`, `_3`, and so on appended. This also covers fallback collisions, since duplicate offsets are allowed.

### \_list Files to options/\*.json5

`_list` files have a count on line 1, then `value label` pairs:

```text
3
0x00 No
0x01 Yes, Aquatic
0x02 Yes, Lavatic
```

Converts to:

```json5
[
    { value: 0x00, label: 'No' },
    { value: 0x01, label: 'Yes, Aquatic' },
    { value: 0x02, label: 'Yes, Lavatic' }
]
```

The declared count should match the number of parsed pairs. A mismatch is a conversion warning.

A line containing only a value and no label converts to `label: 'unknown'`. This keeps converted files free of empty-label warnings.

### \_record Files to entries/\*.json5

`_record` files are positional: one label per line, no explicit index.

```text
<<Nothing>>
Warrior
Archer
```

Converts to:

```json5
[
    { value: 0x00, label: '<<Nothing>>' },
    { value: 0x01, label: 'Warrior' },
    { value: 0x02, label: 'Archer' }
]
```

Automated conversion should keep all positional entries, including placeholders, so indexes remain inspectable.

### \_name Files to options/\*.json5

`_name` files have a count on line 1, then `value label` pairs. They convert exactly like `_list` files and are used as dropdown option files.

```text
256
0x0000 nothing
0x0001 Warrior
0x0002 Archer
```

## Future Extensions

These features are intentionally outside version `1`:

1. Bit-level fields.
2. Character mapping tables for non-ASCII text.
3. Directory targets.
4. Multi-file modules where one logical table is split across several files.
5. Conditional field visibility.
6. Computed fields.
7. Cross-file `count_from` sources, where a table's count lives in a different file than the table.
8. Read-only fields.
