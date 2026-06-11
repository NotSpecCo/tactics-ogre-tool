# Module Spec

Modules define how to read and edit structured binary tables inside selected game `.dat` files. Each module is a `.json5` file loaded from the filesystem at runtime.

This document defines version `1` of the module format.

## Index

- [File Conventions](#file-conventions)
- [Directory Structure](#directory-structure)
- [Path Rules](#path-rules)
- [Module File Format](#module-file-format)
- [Module Keys](#module-keys)
- [Entry Definition](#entry-definition)
    - [Block Headers](#block-headers)
    - [Header-Driven Tables](#header-driven-tables)
    - [Fixed-Count Tables](#fixed-count-tables)
    - [Example: Battle Unit Tables](#example-battle-unit-tables)
- [Field Types](#field-types)
- [Numeric Rules](#numeric-rules)
- [Text Rules](#text-rules)
- [Duplicate and Overlapping Storage](#duplicate-and-overlapping-storage)
- [Options Files](#options-files)
- [Entry Files](#entry-files)
- [Validation Rules](#validation-rules)
- [Future Extensions](#future-extensions)

## File Conventions

Module and sidecar files parse with a standard JSON5 parser. Strings are always quoted, so labels like `No` or `Tactics Ogre: Reborn` need no special handling. JSON5 comments (`//` and `/* */`) are permitted and ignored by loaders; tools that rewrite files are not required to preserve them. Text that must survive belongs in `notes`. All property names and module file names are lowercase `snake_case`.

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

The user selects a `.dat` file. The app decrypts and unpacks that file in the background, keeps the resulting payload in memory, and applies module offsets to that in-memory payload. Each supported `.dat` must resolve to exactly one editable payload. If the selected `.dat` resolves to zero payloads, multiple editable payloads, or an unpacking shape the app cannot map to one payload, the app must throw an error and leave UI presentation of that error to the caller. `base_offset` and field `offset` values are never relative to the encrypted `.dat` container or the intermediate zip wrapper.

Sidecar paths such as `entry.labels_file` and `options_file` are relative to the module file's directory and must stay within the module tree after normalization.

## Module File Format

```json5
{
    schema_version: 1,
    id: 'battle_armament',
    label: 'Armament',
    notes: 'Weapon and armor stats.',

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
| `entry`          | yes      | Block header flag, entry count, size, and optional label sidecar.               |
| `fields`         | yes      | Ordered field and section definitions.                                          |

`id` values are persistence keys. They must not be generated from `label` at load time.

`notes` is optional help text shown to the user in the UI. Anything that has a `label` may also have `notes`: modules, fields (including sections), options, and entries.

`source` is an optional free-form object recording where a module definition came from (for example, the editor module it was originally converted from). The app never reads it.

## Entry Definition

### Block Headers

Most tables in the target files are immediately preceded by a 16-byte block header:

| Offset | Size | Content                                                                     |
| ------ | ---- | --------------------------------------------------------------------------- |
| `0x00` | 4    | Magic: the ASCII bytes `xlce`                                               |
| `0x04` | 4    | Entry count                                                                 |
| `0x08` | 4    | Data start offset relative to the header. Always `0x10` in the known files. |
| `0x0c` | 4    | Entry size in bytes                                                         |

Header integers use the module's `endian`. Table data begins at the end of the header, so `base_offset` (the address of the first entry) is the header address plus `0x10`.

### Header-Driven Tables

A table preceded by a block header sets `header: true`:

```json5
entry: {
    header: true,
    count: 256,
    size: 208,
    labels_file: 'entries/class.json5'
},
```

With `header: true`, the block header is at `base_offset - 0x10` and the entry count is read from the header at runtime. The header count is authoritative. This is how the battle unit tables, whose counts vary per file, resolve their real count, and it means a table edited to a non-vanilla length still loads with the correct number of entries.

`count` is optional on header-driven tables. When present it is the expected count: if the header count differs, the reader uses the header count and surfaces a divergence warning to the caller. The warning tells the user the file diverges from the count the module author expected without blocking the edit.

`size` is always required. It is the layout contract for `fields`: every field offset is authored against this entry width. The reader must verify the header's entry size equals `entry.size` and throw an error on mismatch, since a different stored width means the module's field layout does not apply to that file.

### Fixed-Count Tables

A table with no preceding block header omits `header` (or sets `header: false`) and must declare a fixed `count`:

```json5
entry: {
    count: 1,
    size: 0x10,
    labels_file: 'entries/battle_unit_header.json5'
},
```

Fixed-count tables cover two cases:

1. Tables not preceded by an `xlce` block header.
2. Deliberate partial views that expose fewer entries than the block holds. For example, `battle_hit_rate_configuration` shares its block with the full `battle_hit_rate` table but exposes only entry `0`, as a configuration record with its own field layout. A partial view stays fixed-count even when its data sits inside a header-driven block.

`labels_file` is optional on both forms. Omitting it and setting it to `null` are equivalent: the table has no entry labels.

### Example: Battle Unit Tables

The battle unit modules show both forms working together. `battle_entry_battle_unit` is a header-driven table over a glob target. Per-file counts vary, so no expected `count` is declared:

```json5
files: ['battle/entry/entry_unit_*.dat'],
base_offset: 0x30,
entry: {
    header: true,
    size: 0xc4,
    labels_file: null
},
```

`battle_entry_battle_unit_header` exposes that same block header as an editable one-entry table at `0x20`. No `xlce` header precedes it, so it is a fixed-count table:

```json5
files: ['battle/entry/entry_unit_*.dat'],
base_offset: 0x20,
entry: {
    count: 1,
    size: 0x10,
    labels_file: 'entries/battle_unit_header.json5'
},
```

Note that editing this module edits the battle unit block header itself, including the count that header-driven reads of the battle unit table resolve.

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

Any other form is invalid where the schema expects an integer: leading `-` signs, fractional parts, exponents, `Infinity`, `NaN`, booleans, nulls, arrays, objects, and strings containing numbers such as `"256"` or `"0x0100"`. A leading `+` sign is permitted because standard JSON5 parsers accept it; `+256` parses to `256`. After parsing, the normal validation rules still apply, such as non-negative, positive, and stored-range checks.

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

## Duplicate and Overlapping Storage

Duplicate offsets are allowed. A module may intentionally expose the same stored value through multiple interpretations, such as a raw numeric field plus one or more dropdowns.

Each field is an independent view over its declared storage. Editing any field writes that field's bytes. Other fields that read the same bytes must be refreshed from storage after the write. There is no field priority; the last successful write wins.

Overlap rules:

1. Identical storage ranges are allowed.
2. Partially overlapping byte ranges are allowed but produce a validation warning, since they are usually offset mistakes.
3. A `section` never participates in overlap checks.

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
4. Placeholder labels such as `<not used>` or `<<Nothing>>` are ordinary labels. Keeping them preserves the meaning of the surrounding numeric indexes.
5. Entry values at or above the resolved entry count are allowed but never displayed; the UI ignores them. This supports sharing one entry file between modules with different counts, such as `entries/class.json5` used by both `battle_class` (count 256) and `menu_classmark` (count 80).

## Validation Rules

A module set is valid only if all modules and sidecars pass these checks.

Module validation:

1. `schema_version` is `1`.
2. Module `id` is unique across the loaded module set.
3. Module `id` and field `id` values match `^[a-z][a-z0-9_]*$`.
4. `files` is non-empty, and every item is a relative POSIX path or glob.
5. `files` entries do not contain `..`, do not begin with `/`, and do not resolve outside the game root.
6. `base_offset` is a non-negative integer, and at least `0x10` when `entry.header` is `true`.
7. `endian` is `little` or `big`.
8. `entry.size` is a positive integer.
9. `entry.header`, when present, is a boolean.
10. `entry.count` is required unless `entry.header` is `true`. When present, it is a positive integer.
11. `fields` is non-empty.
12. Field IDs are unique within a module.

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
8. Entry values at or above a referencing module's `entry.count`, when one is declared, are warnings, not errors.

Runtime payload validation:

1. For header-driven tables, the block header range `base_offset - 0x10` through `base_offset` must be within the decrypted/unpacked in-memory payload length.
2. For header-driven tables, the block header magic must be the ASCII bytes `xlce`, the data start offset must be `0x10`, and the entry size must equal `entry.size`.
3. The resolved entry count is the block header count for header-driven tables and `entry.count` for fixed-count tables. `base_offset + resolved_count * entry.size` must be within the matched target payload length.
4. Every stored field range for every resolved entry, `base_offset + entry_index * entry.size + field.offset` through `size`, must be within the matched target payload length.

If any runtime payload validation check fails, the reader or writer must throw an error and leave UI presentation of that error to the caller.

A divergence between a header-driven table's optional expected `entry.count` and the block header count is not a validation failure. The reader uses the header count and reports a non-fatal warning to the caller, which owns presenting it to the user.
