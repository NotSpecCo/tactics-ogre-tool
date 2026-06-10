# Module System Migration Plan

Migrate the app from hardcoded Rust module definitions to the JSON5 module format defined in `docs/MODULE-SPEC.md`. This document is the working plan: an agent should complete **one step at a time, in order**, verifying and committing after each step.

**Status:** All 11 steps are complete and committed on `fixes-part-1`, including the manual end-to-end checklist. The Follow-up tasks below are all resolved (pending review). The migration is done.

> **Post-migration spec change (2026-06-10):** after this plan completed, the spec replaced `count_from` with block-header-driven tables (`entry.header: true`, reading entry counts from the 16-byte `xlce` block headers; see the spec's Entry Definition and Header-Driven Table Detection sections). The converter, Rust module layer, committed `modules/` tree, and frontend were migrated to the header format the same day, including a non-fatal expected-count divergence warning surfaced through `ModuleSummary`. References to `count_from` below describe what was built at the time, not the current code.

**How to use this document (for the executing agent):**

1. Read `docs/MODULE-SPEC.md` in full before starting any step. It is the authority on all format and behavior questions. This plan never overrides the spec; if they conflict, the spec wins — and flag the conflict to the user immediately.
2. Complete the single step you were asked to do. Do not start the next step.
3. Run that step's verification, plus the global conventions below.
4. Do not commit. When the step is verified, stage the changes if useful and stop for user review; the user commits (or asks for a commit) after approving.

---

## Context

**What exists today (legacy, to be replaced):**

- `src-tauri/src/modules/` — hardcoded Rust module definitions (`classes.rs`, `equipment.rs`, etc.) generated once by `scripts/gen_modules.mjs` from the _old_ `reference_files/nightmare_modules` set. **Ignore these for design purposes.** They get deleted in Step 7.
- `scripts/convert_nmm_modules.mjs` — an earlier converter that targets the correct source set (`reference_files/nightmare_modules_new`) but **emits YAML**, from before the spec switched to JSON5. It also emits keys that no longer exist in the spec (e.g. `description`). It gets rewritten in Step 1.
- `src-tauri/src/pipeline/session.rs` — session state, record read/write against the legacy module shape, Tauri command structs.
- `src-tauri/src/lib.rs` — Tauri commands (`open_dat`, `get_record`, `set_field`, …) bound to the legacy shape.
- `src/lib/tauri.ts`, `src/lib/stores/*.svelte.ts`, `src/lib/components/*` — frontend bound to the legacy IPC types (`ModuleSummary`, `Record`, `FieldType` of `Uint | Int | Dropdown | Hex`).
- `modules/entries/` and `modules/options/` — empty directories, leftovers. The converter will populate `modules/`.

**What stays:**

- The decrypt/unpack pipeline (`src-tauri/src/crypto`, `src-tauri/src/pipeline/dat.rs`, `src-tauri/src/filetable`) is unchanged. Modules operate on the in-memory unpacked payload (`PackData.bytes`) it produces, exactly as the spec's Path Rules section describes.
- The session model (open game directory → open one dat → edit → save) stays. Only the module layer and its IPC surface change.

**Key references:**

| Path                                     | What it is                                                                                                                             |
| ---------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------- |
| `docs/MODULE-SPEC.md`                    | The spec. Authoritative for everything.                                                                                                |
| `reference_files/nightmare_modules_new/` | Source of truth for conversion: 24 `.nmm` (20 `battle/`, 2 `battle/entry/`, 2 `menu/`), 75 `_list/`, 10 `_name/`, 18 `_record/` files. |
| `reference_files/nightmare/doc/`         | Nightmare format docs, historical background only.                                                                                     |
| `modules/`                               | Output directory for the converted JSON5 module set (committed to the repo).                                                           |
| `src-tauri/src/pipeline/dat.rs`          | `unpack_dat` / `pack_dat` — produces the payload modules read/write.                                                                   |

---

## Critical Facts and Gotchas

Things an agent is likely to get wrong without being told:

1. **The spec is clean-slate.** Do not copy patterns from `src-tauri/src/modules/` or from Nightmare's own loader. The new Tactics Ogre module set in `reference_files/nightmare_modules_new` is authoritative for layouts; older module sets are historical.
2. **Offsets are payload-relative.** `base_offset`, field `offset`, and `count_from` offsets apply to the decrypted+unpacked in-memory payload, never to the encrypted `.dat` or the zip wrapper.
3. **Text fields are lossy, one-way.** Reads decode to `U+FFFD` outside printable ASCII; the app must never write a text field by re-encoding its displayed value — only from explicit user input. Cover both in tests (the reference set has only one `TEXT` field, so this path is easy to miss).
4. **Duplicate/overlapping storage is intentional.** Multiple fields may view the same bytes. After any field write, every other field reading those bytes is stale — the UI must re-read the record from the backend after each successful write rather than patching local state.
5. **Unmatched dropdown values are valid.** A stored value with no matching option displays as the raw number in the field's `display` format and must never be coerced to an option on load or save.
6. **Entry label lookup is by `value`, not position.** Entry values ≥ the module's resolved count are allowed in the file but never displayed (one entry file can serve modules with different counts, e.g. `entries/class.json5` for both count 256 and count 80).
7. **Glob matching is case-insensitive and segment-scoped.** `*` and `?` never match `/`. Use `globset` with `literal_separator(true)` and `case_insensitive(true)`.
8. **The committed set validates with zero errors and exactly six warnings** (pinned by the golden test in `module_set/tests.rs`): the two known partial overlaps (`battle_armament` and `battle_class`, a 1-byte set-reference dropdown overlapping a 2-byte padding field at offset `0x00`), three duplicate-option-value alias warnings (`options/skill.json5`, `options/armament_set_ref.json5`, `options/class_set_ref.json5` — faithful to the Nightmare sources), and one entry-count warning (`menu_classmark` referencing the shared 256-entry `entries/class.json5` with fixed count 80, the sharing case the spec explicitly supports).
9. **Converter line handling:** split on `\n`, `\r\n`, or `\r` (mixed allowed), trim trailing whitespace per line. The first three comment lines of each `.nmm` (checksum, ID address, ID string) plus the blank BASEPOINTER line are Nightmare verification metadata — skipped, never converted to `notes`.
10. **ID generation has fallbacks and dedup.** Empty/non-letter-leading normalized labels fall back to `field_0x<offset>` (lowercase hex, ≥2 digits) or `section_<ordinal>`; then deduplication appends `_2`, `_3`, … in field order across _all_ IDs including fallbacks.
11. **JSON5 numeric handling.** The `json5` crate parses every number as `f64` and saturating-casts into integer targets — it silently accepts `-1` as `0`, `16.5` as `16`, and `Infinity` as `u64::MAX`. `module_spec::parse` therefore runs a token-level numeric-literal pre-pass (`validate_numeric_literals`) that rejects any literal not matching optional `+` followed by unsigned decimal or hex, plus `Infinity`/`NaN` in value position, before deserializing. `+5` parses as `5`, which the spec explicitly permits. Strings, booleans, arrays, and objects in integer positions are rejected by serde as usual.
12. **`count_from` reads from the currently matched target file**, with its own range validation _before_ the count is read, and re-validation of the full table extent after. `count_from.type` must be `uint`; `size` 1/2/4; `base_offset` defaults to 0.
13. **3-byte integers exist** (`uint`/`dropdown` size 3, both endians). The legacy code never handled these.
14. **Errors vs warnings is load-bearing.** Partial overlaps, duplicate option values, empty labels, and entry values ≥ fixed count are _warnings_. Everything else in the validation rules is an _error_. Runtime payload violations are errors thrown to the caller; UI presentation is the caller's job.
15. **`id` values are persistence keys** — never derived from `label` at load time. The converter generates them once; the loader only validates them.
16. **User preferences from project memory:** thorough Rust unit tests for everything (the user is less familiar with Rust); run `cargo fmt` after every Rust change; run `cargo clippy` per feature; flag spec inaccuracies immediately instead of working around them.

---

## Decisions

Made during planning; revisit only with the user.

| Decision                   | Choice                                                                                                                                                                                                                     | Rationale                                                                                                                                   |
| -------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------- |
| Converter language         | Node (`scripts/`), rewrite in place                                                                                                                                                                                        | Dev-time tool, already half-exists, no reason to pay the Rust cost. Rust only validates/consumes JSON5.                                     |
| Module set distribution    | Committed to repo at `modules/`, bundled via Tauri `resources`, resolved at runtime through `resource_dir()`; env var `TO_TOOL_MODULES_DIR` overrides for dev/testing                                                      | Spec says modules load from the filesystem at runtime; bundling keeps the app self-contained while letting modders point at their own tree. |
| JSON5 parsing crate        | `json5` (serde-based)                                                                                                                                                                                                      | Mature, deserializes straight into serde types, supports hex literals and comments. See gotcha 11 for the one known gap.                    |
| When modules load          | Once at app startup into `AppState`, with a reload command                                                                                                                                                                 | Module errors should be visible before any dat is opened; `files` matching computes per-dat applicability.                                  |
| Write semantics            | `set_field` validates + writes, then the frontend re-fetches the record                                                                                                                                                    | Simplest correct handling of overlapping views (gotcha 4).                                                                                  |
| Converted JSON5 formatting | Single-quoted strings, 4-space indent, unquoted keys; `base_offset`, field `offset`, `count_from` offsets, option/entry `value` in lowercase `0x` hex; counts and sizes always decimal (matches the spec's module example) | Deterministic output → meaningful diffs and a CI determinism check.                                                                         |

---

## Global Conventions (every step)

- The Cargo manifest lives at `src-tauri/Cargo.toml`, not the repo root. Run all `cargo` commands from `src-tauri/` (or pass `--manifest-path src-tauri/Cargo.toml`).
- Rust changes: `cargo fmt` always; `cargo clippy --all-targets -- -D warnings` and `cargo test` before finishing a step.
- Frontend changes: `npm run check`, `npm run lint`, `npm run test:unit` before finishing a step.
- New Rust code gets thorough unit tests in the same step — error paths and boundary values, not just happy paths.
- If a step reveals a spec problem (ambiguity, contradiction, reference-file mismatch), stop and surface it; check `reference_files/` before proposing any spec change.

---

## Steps

### Step 1 — Rewrite the NMM → JSON5 converter

**Goal:** `scripts/convert_nmm_modules.mjs` emits spec-compliant JSON5 instead of YAML.

**Tasks:**

- Restructure into `scripts/lib/nmm_convert.mjs` (pure functions: parsing, normalization, conversion, JSON5 serialization) and a thin CLI wrapper at `scripts/convert_nmm_modules.mjs` keeping the existing flags (`--source`, `--out`, `--clean`, `--dry-run`) and adding `--check` (re-convert and diff against the output dir; nonzero exit on any difference; used by CI).
- Implement every rule in the spec's "Converting Nightmare Modules" section: directory mapping (`_record`→`entries/`, `_list`/`_name`→`options/` with `name_` prefix for `_name`), name normalization, line splitting per gotcha 9, metadata skipping, comments→notes by position, 7-line header conversion, title-segment→`files` mapping, type-code table, `separator.txt`→`section`, sidecar path resolution relative to the source `.nmm`, fallback IDs and dedup (gotcha 10), `_list` count-mismatch warning, value-only lines → `label: 'unknown'`, `BattleUnit` `count_from` special case exactly as specified.
- Drop all keys not in the spec (the old converter emitted `description`; the spec has only `notes`).
- Emit JSON5 with the formatting convention from the Decisions table. Serialization must be deterministic.
- Vitest coverage for the pure functions: normalization examples straight from the spec (`ItemType`, `Skill: <not used> 0006`, `? Flag ?`, `??`), header conversion, comment placement, each type code, section conversion, fallback/dedup IDs, `_list`/`_record`/`_name` conversion, mixed newline handling.

**Verify:** `npm run test:unit` passes; `npm run convert:nmm -- --dry-run` reports all 24 modules and all sidecars without errors.

### Step 2 — Generate and commit the module set

**Goal:** A committed, spot-checked `modules/` tree.

**Tasks:**

- Run `npm run convert:nmm -- --clean` and commit the output.
- Spot-check against the spec and sources (fix the converter, not the output, if anything is off):
    - `modules/battle_armament.json5`: `files: ['battle/battle_data_release.dat']`, `base_offset: 0x00393f20`, count 761, size 152, has dropdown + section fields, labels `entries/armament.json5`.
    - `modules/battle_entry_battle_unit.json5`: `files: ['battle/entry/entry_unit_*.dat']`, `count_from` block matching the spec example exactly.
    - `modules/battle_entry_battle_unit_header.json5`: fixed count 1, size 16 (`0x10` in source), `base_offset: 0x20`.
    - `modules/menu_shop.json5` / `modules/menu_classmark.json5`: `files: ['menu/menu_data.dat']`.
    - `options/name_*.json5` files exist for all 10 `_name` sources; `options/class.json5` and `options/name_class.json5` both exist (collision case).
    - No `options/separator.json5` was emitted.
    - The single `TEXT` field in the set converted to `type: 'text'`.
- Add `convert:nmm:check` npm script running `--check`.

**Verify:** `npm run convert:nmm:check` exits 0; re-running conversion produces no diff.

### Step 3 — Rust: module spec types and JSON5 parsing

**Goal:** `src-tauri/src/module_spec/` parses module and sidecar files into typed structs. No I/O orchestration yet — given a string, produce a struct or a precise error.

**Tasks:**

- Add the `json5` crate to `src-tauri/Cargo.toml`.
- Define serde types mirroring the spec: `ModuleFile` (`schema_version`, `id`, `label`, `notes`, `source`, `files`, `base_offset`, `endian` default little, `entry`, `fields`), `Entry` (`count` xor `count_from`, `size`, `labels_file` where absent ≡ null), `CountFrom`, `Field` (tagged by `type`: uint/int/bytes/text/dropdown/section, with `display`, `endian`, `notes`, `options_file` as applicable), and the sidecar item shape (`value`, `label`, `notes`).
- Parse functions: `parse_module(&str) -> Result<ModuleFile, ParseError>`, `parse_sidecar(&str) -> Result<Vec<SidecarItem>, ParseError>`. These take content only, so `ParseError` carries key/position context (which key or item failed and why); attaching the _file path_ to errors is the Step 4 loader's job, since only it knows which file the content came from.
- Enforce the post-parse numeric rules from gotcha 11 (integral, in-range, non-negative/positive where required) at the type level where serde allows, otherwise in a `validate_shape` pass owned by this module. Unknown keys: reject (`deny_unknown_fields`) — the spec defines a closed format.
- Tests: every field type parses from spec-example snippets; hex and decimal literals; comments ignored; rejection cases (string-where-int, float, negative `base_offset`, both `count` and `count_from`, unknown keys, missing required keys, top-level non-array sidecar).

**Verify:** `cargo test` green; clippy clean.

### Step 4 — Rust: module set loader and validator

**Goal:** Load a `modules/` directory into a validated `ModuleSet` with an error/warning report.

**Tasks:**

- In `module_spec` (or a sibling `module_loader` module): walk the modules dir for top-level `*.json5`, parse each, resolve and load sidecars (`labels_file`, `options_file`) relative to the module file's directory, rejecting paths that escape the module tree after normalization.
- Implement the spec's full validation rule list: 13 module rules, 13 field rules, 8 sidecar rules. Model results as `ValidationReport { errors: Vec<Issue>, warnings: Vec<Issue> }` where each `Issue` names the file, module/field id, and rule. Errors vs warnings exactly per spec (gotcha 14).
- Overlap detection between non-section fields: identical ranges OK, partial overlap → warning.
- Sidecar cross-checks: option values fit every referencing dropdown's size; entry values ≥ a referencing module's fixed count → warning.
- Loading continues past per-module errors so the report covers the whole set (a broken module doesn't hide others).
- Tests: fixture-based (small inline module trees under `tempfile` dirs) for each rule firing and not firing; plus the **golden test** — load the real committed `../modules/` set and assert zero errors and exactly the two known overlap warnings (gotcha 8).

**Verify:** `cargo test` green including the golden test; clippy clean.

### Step 5 — Rust: target file matching

**Goal:** Given a module set and a game-root-relative dat path, return the modules that apply. This is pure filtering of the already-loaded in-memory set (all modules load once at app startup — see Decisions); no JSON5 files are read here or when a dat is opened.

**Tasks:**

- Add `globset`. Compile each module's `files` patterns once at load with `literal_separator(true)`, `case_insensitive(true)`.
- Validate patterns per spec at load time (relative, no leading `/`, no `..`, files-not-directories) — fold these checks into Step 4's validator if cleaner.
- `ModuleSet::modules_for(dat_path) -> Vec<&LoadedModule>`, normalizing the input to POSIX separators.
- Tests: literal match, `entry_unit_*` glob, case-insensitivity (`ENTRY_UNIT_0001.DAT`), `*` not crossing `/`, `?` single-char, non-matching paths.

**Verify:** `cargo test` green; clippy clean.

### Step 6 — Rust: runtime reader and writer

**Goal:** Read records from and write fields into a payload byte buffer per a loaded module. The most test-heavy step.

**Tasks:**

- New module (suggested `src-tauri/src/module_runtime/`):
    - `resolve_count(payload, module)` — fixed count passthrough; `count_from` with pre-read range validation, endian handling, and post-resolution table-extent validation (spec runtime rules 1–3).
    - `read_record(payload, module, index)` — all field types: `uint`/`int` sizes 1/2/3/4 (int: 1/2/4) both endians, `bytes` as raw vec, `text` with lossy `U+FFFD` decode and first-NUL termination, `dropdown` as uint. Sections pass through as headings with no value.
    - `write_field(payload, module, index, field_id, value)` — range checks per the stored-range table; `text` accepts printable ASCII only, ≤ size, NUL-padded; `bytes` requires exact declared length; rejects writes to `section`; runtime rule 4 bounds check before touching bytes.
    - All failures are typed errors; no UI strings.
- Tests (synthetic payloads built in-test): round-trips for every type/size/endian combo; 3-byte LE and BE byte order asserted byte-for-byte; boundary values (0, max, min, max+1 rejected, -129 rejected, …); text lossy decode, no-terminator full-size read, embedded NUL, rejection of non-ASCII and over-length writes, NUL padding asserted; the no-round-trip rule (reading then writing back a lossy string must not be possible via any API that takes a read result); `count_from` happy path + count region out of bounds + resolved table out of bounds; overlapping fields both readable, write through one visible through the other; index ≥ count rejected.

**Verify:** `cargo test` green; clippy clean.

### Step 7 — Tauri integration: replace the legacy module layer

**Goal:** The backend serves JSON5 modules end to end; legacy module code is gone.

**Tasks:**

- Bundle `modules/` as a Tauri resource (`tauri.conf.json` `bundle.resources`); resolve at startup via `resource_dir()`, overridable with `TO_TOOL_MODULES_DIR`. Load the `ModuleSet` + `ValidationReport` into `AppState` at startup; modules with errors are excluded from use but remain in the report.
- Command changes in `src-tauri/src/lib.rs` + `pipeline/session.rs`:
    - `open_dat` → matches modules to that dat via Step 5, stores the matched set (with counts resolved by running `count_from` against the freshly unpacked payload) **in `DatSession`**, and returns it with entry labels.
    - `get_record` / `set_field` → resolve module IDs **only through the matched set stored in `DatSession`**, never through a global module lookup; a module ID that didn't match the open dat is an error. (The legacy commands do global `modules::find_module` lookups — do not preserve that.) Add tests that a loaded-but-unmatched module ID is rejected by both commands.
    - `get_record` → new field payload: id, label, notes, type, size, display, value variant (`uint`/`int`/`bytes`/`text`), options (value/label/notes) for dropdowns, sections in field order.
    - `set_field` → validates + writes via Step 6, marks dirty, returns the **re-read record** so the frontend always refreshes overlapping views in one IPC call.
    - New `get_module_diagnostics` returning the `ValidationReport` and module-set source path; new `reload_modules`.
    - `scan_directory`'s `has_modules` flag computed from the loaded set's file matching.
- Delete `src-tauri/src/modules/` entirely, `scripts/gen_modules.mjs`, and the `get_all_modules` command; remove `pub mod modules` from `lib.rs`.
- Update/replace session-level tests that referenced legacy modules.

**Verify:** `cargo test` green; clippy clean; `npm run build:debug` compiles (frontend will still be broken-at-runtime until Step 8 — that's expected; note it in the commit message).

### Step 8 — Frontend: IPC types and stores

**Goal:** `src/lib/tauri.ts` and the stores speak the new IPC shape.

**Tasks:**

- Rewrite the IPC types in `src/lib/tauri.ts` to mirror Step 7's structs exactly (module summary with id/label/notes/resolved count/entry labels keyed by value; record with ordered fields incl. sections; field value variants; diagnostics report).
- `session.svelte.ts`: modules come from `open_dat`'s matched list (drop the `getAllModules` call); expose diagnostics state.
- `editor.svelte.ts`: entry selection by index with label lookup by value (fall back to numeric index; ignore label values ≥ resolved count); `updateField` replaces `currentRecord` with the record returned by `set_field` (gotcha 4) while preserving dirty-field tracking semantics.
- Update `npm run check` to green — components may need minimal mechanical prop renames here to keep the tree compiling, but visual/behavioral work belongs to Step 9.

**Verify:** `npm run check`, `npm run lint`, `npm run test:unit` green.

### Step 9 — Frontend: field rendering for the new format

**Goal:** The editor UI renders every spec construct correctly.

**Tasks:**

- `FieldForm` / `RecordFieldInput` / `DropdownField` (restructure as needed):
    - `section` fields render as visual headings.
    - `notes` render as help text/tooltips wherever present (module, field, option, entry).
    - `uint`/`dropdown` honor `display`: hex values shown `0x`-prefixed; user input accepted in both decimal and hex (`0x` prefix) regardless of display; input validated against the field's stored range before invoking `set_field`.
    - `int` always decimal, range-validated.
    - `dropdown` shows the canonical (first-matching) option label; an unmatched stored value renders as the raw number in display format with the options list still selectable (gotcha 5); duplicate values appear as listed.
    - `bytes` renders as a fixed-length hex byte editor (length must equal `size` to save).
    - `text` is an input with maxlength = size, printable-ASCII validation, and a hint that non-ASCII reads are display-only; never auto-writes the displayed value back (gotcha 3).
- `RecordSelector`: labels by value with numeric fallback; empty-label fallback to numeric per spec.
- Run the Svelte MCP autofixer on changed components per project config.

**Verify:** `npm run check`, lint, unit tests green; manual smoke via `npm run dev` against a game directory if available.

### Step 10 — Frontend: module diagnostics surface

**Goal:** Module load problems are visible without opening a terminal.

**Tasks:**

- A diagnostics view (modal or panel reachable from the main screen) listing `ValidationReport` errors and warnings with file, module id, and rule text; a "reload modules" action wired to `reload_modules`.
- Startup behavior: if the module set has errors, show a non-blocking notice (the app still works for valid modules); the two known overlap warnings display as warnings, not alarming errors.
- File tree: dats with no matching modules render `has_modules: false` styling as today.

**Verify:** checks green; manually verify by temporarily breaking a module file in a dev modules dir (`TO_TOOL_MODULES_DIR`) and confirming the report renders, then restoring.

### Step 11 — Cleanup and full verification

**Goal:** No legacy remnants; everything green; docs current.

**Tasks:**

- Repo sweep for dead references: legacy `ModuleSummary`/`FieldType` shapes, `gen_modules`, `get_all_modules`, old `reference_files/nightmare_modules` mentions in active code or scripts (reference files themselves stay).
- Update `README.md` (module system description, converter usage, `TO_TOOL_MODULES_DIR`) and `CLAUDE.md` if commands changed.
- Full gate (cargo commands from `src-tauri/`): `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `npm run convert:nmm:check`, `npm run check`, `npm run lint`, `npm run test:unit`, `npm run build:debug`.
- Manual end-to-end checklist with a real game directory (user assists if no dump is on hand):
    1. Open game dir → tree shows module-capable dats.
    2. Open `battle/battle_data_release.dat` → battle modules listed; armament shows 761 entries with labels.
    3. Edit a uint, an int, a dropdown (incl. selecting away from and back to an unmatched value), a bytes field, the text field; save; re-open; values persisted.
    4. Open an `entry_unit_*.dat` → BattleUnit count resolves from the header; entry count matches the header field value.
    5. Overlap pair in armament/class: editing one view refreshes the other.

**Verify:** every command in the full gate passes; checklist complete.

---

## Future work (explicitly out of scope)

Tracked by the spec's Future Extensions section: bit-level fields, character mapping tables, directory targets, multi-file tables, conditional visibility, computed fields, cross-file `count_from`, read-only fields. Do not design for these speculatively.

---

## Follow-up tasks (cleanup after all steps complete)

Known issues deliberately deferred so the steps could proceed. **All resolved.**

1. **Stale `resolved_count` after editing a dynamic count.** ✅ Resolved by dropping the snapshot entirely: `DatSession.modules` stores `Arc<LoadedModule>` and `module_summary`/`summarize_modules` resolve counts against the live payload. `get_modules` returns live summaries (now as `DatSessionInfo`, including modules whose counts edits have broken), and the frontend refreshes summaries after every successful `set_field`.
2. **Silent size fallbacks in `module_runtime` helpers.** ✅ `max_unsigned`/`signed_range`/`sign_extend` now panic on sizes outside the spec's sets, with tests covering valid and panicking sizes.
3. **Per-call deep clones of the module set.** ✅ `ModuleSet.modules` is `Vec<Arc<LoadedModule>>`; set clones and match results are pointer copies.
4. **Triple-nested access chains.** ✅ `MatchedModule` was removed outright (see item 1); call sites use `Arc<LoadedModule>` with a `LoadedModule::id()` accessor.
5. **Duplicate `Endian` enums.** ✅ `binary::Endian` is now a re-export of `module_spec::Endian`.
6. **`npm run check` warns `Cannot find type definition file for 'node'`.** ✅ Added `@types/node`; also removed a `@ts-expect-error` in `vite.config.js` that the missing types had been masking. svelte-check is now fully clean.
