# Tactics Ogre Tool

A tool to help you mod Tactics Ogre Reborn.

**Warning:** This app is a proof of concept and very little testing has been done as of now. Use at your own risk!

## Building

### Prerequisites

- NodeJS v24+
- Rust

### Commands

```shell
# Install node dependencies
npm install

# Build and run development app
npm run dev

# Build production app
npm run build:standalone
```

## Modules

The editor is driven by JSON5 module files in `modules/` that describe the record tables inside the game's `.dat` files (offsets, field types, dropdown options, entry labels). The format is documented in `docs/MODULE-SPEC.md`. The set is bundled with the app; set the `TO_TOOL_MODULES_DIR` environment variable to load a different module directory instead (useful for developing your own modules). Load problems are shown in the app under **Modules**, which also has a reload action so you can edit module files without restarting.

The committed set is generated from the Nightmare module definitions in `reference_files/nightmare_modules_new`:

```shell
# Regenerate modules/ from the Nightmare sources
npm run convert:nmm -- --clean

# Verify modules/ matches a fresh conversion (used by CI)
npm run convert:nmm:check
```
