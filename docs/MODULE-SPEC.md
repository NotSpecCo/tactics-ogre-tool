# Module Spec (YAML)

Modules define how to read and edit structured binary tables inside selected game `.dat` files. Each module is a `.yml` file loaded from the filesystem at runtime.

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
|-- battle_armament.yml
|-- battle_hit_rate.yml
|-- menu_shop.yml
|-- options/                # dropdown option files
|   |-- no_yes.yml
|   |-- item_type.yml
|   `-- ...
`-- entries/                # entry label files
    |-- class.yml
    |-- skill.yml
    `-- ...
```

## Path Rules

Module `files` are game-root-relative POSIX paths to `.dat` files. They must not begin with `/`.

```yaml
files:
    - battle/battle_data_release.dat
    - battle/entry/entry_unit_*.dat
```

`files` entries may be literal paths or glob patterns using `*` and `?`. Directory targets are not part of version `1`; a directory path is invalid. A glob matches files only, not directories.

The user selects a `.dat` file. The app decrypts and unpacks that file in the background, keeps the resulting payload in memory, and applies module offsets to that in-memory payload. `base_offset`, field `offset`, and `entry.count_from` offsets are never relative to the encrypted `.dat` container or the intermediate zip wrapper.

Sidecar paths such as `entry.labels_file` and `options_file` are relative to the module file's directory and must stay within the module tree after normalization.

## Module File Format

```yaml
schema_version: 1
id: battle_armament
label: Armament
description: Armament table editor
notes: Weapon and armor stats.

source:
    format: nightmare
    path: battle/Armament.nmm
    title: Tactics Ogre: Reborn ==> battle_data_release / pack ==> Armament (Equipment)

files:
    - battle/battle_data_release.dat

base_offset: 0x00393F20
endian: little

entry:
    count: 686
    size: 152
    labels_file: entries/armament.yml

fields:
    - id: item_type
      label: Item Type
      offset: 0x00
      size: 1
      type: dropdown
      display: hex
      options_file: options/item_type.yml

    - id: attack_power
      label: Attack Power
      offset: 0x04
      size: 2
      type: uint

    - id: weight
      label: Weight
      offset: 0x08
      size: 2
      type: int

    - id: unknown_0a
      label: Unknown Bytes
      offset: 0x0A
      size: 4
      type: bytes

    - id: name
      label: Name
      offset: 0x10
      size: 16
      type: text

    - id: general_flags
      type: section
      label: General Flags
```

## Module Keys

| Key              | Required | Description                                                                     |
| ---------------- | -------- | ------------------------------------------------------------------------------- |
| `schema_version` | yes      | Must be `1`.                                                                    |
| `id`             | yes      | Stable module identifier. Use lowercase ASCII letters, digits, and underscores. |
| `label`          | yes      | Human-readable module name.                                                     |
| `description`    | yes      | Short human-readable description.                                               |
| `notes`          | no       | Longer help text or source commentary.                                          |
| `source`         | no       | Provenance metadata. Ignored by binary readers and writers.                     |
| `files`          | yes      | Game-root-relative file paths or glob patterns this module applies to.          |
| `base_offset`    | yes      | Byte offset of the first table entry in each matched target file.               |
| `endian`         | no       | `little` or `big`. Defaults to `little`.                                        |
| `entry`          | yes      | Entry count, size, and optional label sidecar.                                  |
| `fields`         | yes      | Ordered field and section definitions.                                          |

`id` values are persistence keys. They must not be generated from `label` at load time.

## Entry Definition

Most tables have a fixed entry count:

```yaml
entry:
    count: 256
    size: 208
    labels_file: entries/class.yml
```

Tables whose count is stored in the target file use `count_from`:

```yaml
entry:
    count_from:
        base_offset: 0x20
        offset: 0x04
        size: 4
        type: uint
    size: 0xC4
    labels_file: null
```

`count` and `count_from` are mutually exclusive.

`count_from.file` is optional. If omitted, the current matched target file is used. If present, it must be a literal game-root-relative file path, not a glob, and must resolve to exactly one file in the same game root. `count_from.base_offset` defaults to `0`. The final count address is `count_from.base_offset + count_from.offset`.

`count_from.type` must be `uint` in version `1`. `count_from.size` must be `1`, `2`, or `4`, and uses the module's `endian` unless `count_from.endian` is provided.

## Field Types

| Type       | Stored value             | Required keys                                           | Notes                                                         |
| ---------- | ------------------------ | ------------------------------------------------------- | ------------------------------------------------------------- |
| `uint`     | Unsigned integer         | `id`, `label`, `offset`, `size`, `type`                 | `size` must be `1`, `2`, or `4`.                              |
| `int`      | Signed integer           | `id`, `label`, `offset`, `size`, `type`                 | `size` must be `1`, `2`, or `4`. Always displayed as decimal. |
| `bytes`    | Raw bytes                | `id`, `label`, `offset`, `size`, `type`                 | `size` may be any positive integer.                           |
| `text`     | Fixed-length ASCII bytes | `id`, `label`, `offset`, `size`, `type`                 | `size` may be any positive integer.                           |
| `dropdown` | Unsigned integer         | `id`, `label`, `offset`, `size`, `type`, `options_file` | `size` must be `1`, `2`, or `4`.                              |
| `section`  | none                     | `id`, `label`, `type`                                   | Visual heading only. No offset, size, or stored value.        |

Common optional field keys:

| Key        | Applies to                  | Description                                                                             |
| ---------- | --------------------------- | --------------------------------------------------------------------------------------- |
| `display`  | `uint`, `dropdown`, `bytes` | `decimal` or `hex`. Defaults to `decimal` for `uint` and `dropdown`, `hex` for `bytes`. |
| `endian`   | `uint`, `int`, `dropdown`   | Overrides module `endian`.                                                              |
| `readonly` | stored fields               | If true, the field may be displayed but must not be written. Defaults to false.         |
| `notes`    | all fields                  | Help text or source commentary.                                                         |

`display` affects only presentation and parsing of user input. It does not change the stored bytes.

## Numeric Rules

Numeric YAML values may be decimal (`256`) or hexadecimal (`0x0100`).

Stored integer ranges are determined by `type` and `size`:

| Type                | Size | Range                         |
| ------------------- | ---- | ----------------------------- |
| `uint` / `dropdown` | `1`  | `0` to `255`                  |
| `uint` / `dropdown` | `2`  | `0` to `65535`                |
| `uint` / `dropdown` | `4`  | `0` to `4294967295`           |
| `int`               | `1`  | `-128` to `127`               |
| `int`               | `2`  | `-32768` to `32767`           |
| `int`               | `4`  | `-2147483648` to `2147483647` |

Writers must reject values outside the field's range.

## Text Rules

Version `1` supports fixed-length ASCII text fields only.

Readers decode bytes up to the first `0x00` byte, or the full field size if no terminator is present. Writers encode ASCII, reject strings longer than `size`, and pad the remaining bytes with `0x00`.

Character mapping tables are not part of version `1`.

## Duplicate and Overlapping Storage

Duplicate offsets are allowed. Nightmare modules intentionally expose the same stored value through multiple interpretations, such as a raw numeric field plus one or more dropdowns.

Each field is an independent view over its declared storage. Editing any writable field writes that field's bytes. Other fields that read the same bytes must be refreshed from storage after the write. There is no field priority; the last successful write wins.

Validation must distinguish these cases:

1. Identical storage ranges are allowed.
2. Partially overlapping byte ranges are allowed only with an explicit `overlap: true` field key.
3. A writable field overlapping a `readonly` field is allowed.
4. A `section` never participates in overlap checks.

## Bit-Level Fields

Bit-level fields are not part of version `1`.

The authoritative `reference_files/nightmare_modules_new` set does not use Nightmare's lowercase `b` bit-suffixed offsets or sizes. Future versions can add bit support if broader Nightmare compatibility becomes a goal.

## Options Files

Used by `dropdown` fields. Each option has an integer value, label, and optional description or notes.

```yaml
- value: 0x00
  label: None

- value: 0x01
  label: Sword
  description: Standard one-handed blade

- value: 0x02
  label: Axe

- value: 0x03
  label: Spear
```

Option files are ordered lists, not maps. The loader must preserve file order.

Option semantics:

1. Values may be decimal or hex.
2. Values need not be contiguous.
3. Duplicate values are allowed as aliases. When reading a stored value with multiple labels, the first matching option is the canonical display label.
4. Empty labels are allowed but should produce a validation warning. UIs should fall back to the numeric value when a label is empty.
5. Missing values do not compact or renumber anything.
6. Values must fit in the dropdown field size that references them.

## Entry Files

Used by `entry.labels_file`. Entry files use the same item shape as options files:

```yaml
- value: 0x00
  label: Broad Sword

- value: 0x01
  label: Short Sword

- value: 0x02
  label: Battle Axe
  description: Two-handed axe with high damage
```

The value is the entry index. Entry files are ordered lists, but lookup is by `value`.

Entry semantics:

1. Missing entry indexes are allowed.
2. Missing entries do not compact or renumber later entries.
3. If an entry index has no label, the UI displays the numeric index.
4. Entries marked `<not used>`, `<deleted>`, or `<<Nothing>>` should normally be kept during automated conversion because they preserve source intent. They may be omitted manually only if the numeric index semantics remain unchanged.
5. Entry values must be less than the resolved entry count.

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
11. `fields` is non-empty.
12. Field IDs are unique within a module.

Field validation:

1. Non-section fields have `offset` and `size`.
2. Section fields do not have `offset`, `size`, `options_file`, `display`, `readonly`, or `endian`.
3. Non-section `offset` is a non-negative integer.
4. Non-section `size` is a positive integer.
5. Each non-section field satisfies `offset + size <= entry.size`.
6. `uint`, `int`, and `dropdown` sizes are `1`, `2`, or `4`.
7. `bytes` and `text` sizes may be any positive integer.
8. `int` does not use `display`.
9. `bytes` may only use `display: hex`.
10. `dropdown` has an `options_file`.
11. Non-dropdown fields do not have `options_file`.
12. Sidecar paths exist and are valid relative paths.
13. Partial overlaps require `overlap: true`.

Sidecar validation:

1. Sidecar files parse as YAML arrays.
2. Every item has `value` and `label`.
3. Values are non-negative integers.
4. Labels are strings.
5. Duplicate values are warnings, not errors.
6. Empty labels are warnings, not errors.
7. Option values fit every dropdown size that references the option file.
8. Entry values are less than every fixed entry count that references the entry file.

## Converting Nightmare Modules

The existing `.nmm` modules in `reference_files/nightmare_modules_new/` are the source of truth for field layouts. Each `.nmm` file, along with the three shared reference directories (`_record`, `_list`, `_name`), converts to this YAML format.

### Nightmare Directory Mapping

| Nightmare            | Ours            | Purpose                                        |
| -------------------- | --------------- | ---------------------------------------------- |
| `_record/*.txt`      | `entries/*.yml` | Row labels for entry selectors                 |
| `_list/*.txt`        | `options/*.yml` | Field dropdowns                                |
| `_name/*.txt`        | `options/*.yml` | Field dropdowns that reference game string IDs |
| `battle/*.nmm`       | `modules/*.yml` | Module definitions                             |
| `battle/entry/*.nmm` | `modules/*.yml` | Battle entry module definitions                |
| `menu/*.nmm`         | `modules/*.yml` | Module definitions                             |

`_list` and `_name` both become `options/` files. `_record` is the only source that becomes `entries/`.

### .nmm Parsing Rules

Nightmare allows blank lines and `#`-prefixed comments almost anywhere. The converter reads `.nmm` files as logical lines by discarding blank lines and comment lines before parsing headers or fields.

The first few physical lines in these modules are Nightmare-specific file verification and relocation metadata:

```text
#181ADE14          # 32-bit checksum of the target file
#0x003BA490        # address of an ID string in the target file
#xlce              # ASCII ID string expected at that address
                   # BASEPOINTER flag, blank in these modules
```

This metadata is not represented directly in YAML. It may be preserved under `source` if useful.

Prose comments before the `.nmm` header convert to module `notes`. Comments inside the field list are not converted automatically because they often represent disabled fields or scratch notes.

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

1. `id` is the lowercase snake_case source path without extension, joined by underscores. `battle/Class.nmm` becomes `battle_class`; `battle/entry/BattleUnit.nmm` becomes `battle_entry_battle_unit`.
2. `label` is the final segment after the last `==>`, trimmed.
3. `description` is `${label} table editor`.
4. `source.format` is `nightmare`.
5. `source.path` is the source path relative to `reference_files/nightmare_modules_new`.
6. `source.title` is the full Nightmare title.
7. Numeric header values may be decimal or hex.
8. `entry.labels_file` is the converted `_record` file path, or `null` for `NULL`.
9. The Nightmare character table path is ignored in version `1`.

Target file conversion rules:

| Nightmare title segment                   | YAML `files`                     |
| ----------------------------------------- | -------------------------------- |
| `battle_data_release / pack`              | `battle/battle_data_release.dat` |
| `menu_data / pack`                        | `menu/menu_data.dat`             |
| `battle / entry / entry_unit_#### / pack` | `battle/entry/entry_unit_*.dat`  |

### Dynamic Battle Unit Counts

`battle/entry/BattleUnitHeader.nmm` converts as a normal fixed-count module:

```yaml
files:
    - battle/entry/entry_unit_*.dat
base_offset: 0x20
entry:
    count: 1
    size: 0x10
    labels_file: entries/battle_unit_header.yml
```

`battle/entry/BattleUnit.nmm` uses the header's record count field:

```yaml
files:
    - battle/entry/entry_unit_*.dat
base_offset: 0x30
entry:
    count_from:
        base_offset: 0x20
        offset: 0x04
        size: 4
        type: uint
    size: 0xC4
    labels_file: null
```

### .nmm Field Type Codes

Each Nightmare field is 5 logical lines: label, offset, size in bytes, type code, and options file or `NULL`.

| Code   | Meaning                            | YAML type  | YAML display |
| ------ | ---------------------------------- | ---------- | ------------ |
| `NEDU` | Normal edit, decimal, unsigned     | `uint`     | `decimal`    |
| `NEDS` | Normal edit, decimal, signed       | `int`      | n/a          |
| `NEHU` | Normal edit, hex, unsigned         | `uint`     | `hex`        |
| `NDDU` | Normal dropdown, decimal, unsigned | `dropdown` | `decimal`    |
| `NDHU` | Normal dropdown, hex, unsigned     | `dropdown` | `hex`        |
| `HEXA` | Raw hex byte dump                  | `bytes`    | `hex`        |
| `TEXT` | Fixed-length text bytes            | `text`     | n/a          |

Fields whose options file is `_list/separator.txt` convert to `section` entries. Keep the Nightmare label as the section label and discard the original offset, size, type code, and options file.

Generated field IDs are lowercase `snake_case` labels. If that produces duplicates, append `_2`, `_3`, and so on in field order. If the normalized label is empty or does not start with a letter, use `field_<offset>` or `section_<ordinal>`.

### \_list Files to options/\*.yml

`_list` files have a count on line 1, then `value label` pairs:

```text
3
0x00 No
0x01 Yes, Aquatic
0x02 Yes, Lavatic
```

Converts to:

```yaml
- value: 0x00
  label: No

- value: 0x01
  label: Yes, Aquatic

- value: 0x02
  label: Yes, Lavatic
```

The declared count should match the number of parsed pairs. A mismatch is a conversion warning.

### \_record Files to entries/\*.yml

`_record` files are positional: one label per line, no explicit index.

```text
<<Nothing>>
Warrior
Archer
```

Converts to:

```yaml
- value: 0x00
  label: '<<Nothing>>'

- value: 0x01
  label: Warrior

- value: 0x02
  label: Archer
```

Automated conversion should keep all positional entries, including placeholders, so indexes remain inspectable.

### \_name Files to options/\*.yml

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

## TODOs

These issues should be resolved before treating the version `1` spec as complete:

1. Decide how to represent Nightmare `NEHU` fields with `size: 3`. The authoritative module set has two 3-byte `NEHU` fields, but version `1` currently limits `uint` and `dropdown` fields to sizes `1`, `2`, and `4`.
2. Define deterministic conversion behavior for partial byte overlaps. `battle/Armament.nmm` and `battle/Class.nmm` contain intentional partial overlaps between a one-byte interpreted field and a wider unused/padding field, and the converter needs a rule for when to emit `overlap: true`.
3. Decide target glob case sensitivity. The reference game files include both `entry_unit_*.dat` and `ENTRY_UNIT_*.dat`, so `battle/entry/entry_unit_*.dat` may not match all battle entry files on case-sensitive filesystems.
4. Define runtime payload bounds checks. Validation should say what happens when `base_offset + entry.count * entry.size`, a dynamic `count_from` address, or any field range exceeds the decrypted/unpacked in-memory payload length.
5. Define YAML numeric scalar handling precisely. Examples use unquoted hex values such as `0x00393F20`, but YAML parser behavior can differ; the loader should either require a YAML parser that preserves these as integers or explicitly accept numeric strings.
6. Define the app-level unpacking invariant for selected `.dat` files. The spec says offsets apply to the decrypted/unpacked in-memory payload, but should state whether every supported `.dat` must resolve to exactly one editable payload and what error is produced if it does not.
