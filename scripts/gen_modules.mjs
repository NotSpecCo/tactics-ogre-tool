import { existsSync, readFileSync, writeFileSync } from 'fs';
import { join } from 'path';

const NMM_DIR = join(import.meta.dirname, '../reference_files/nightmare_modules');
const OUT_DIR = join(import.meta.dirname, '../src-tauri/src/modules');

const MODULES = [
    {
        nmm: 'TOReborn Entry Unit.nmm',
        id: 'entry_unit',
        name: 'Entry Unit Editor',
        desc: 'Edit starting unit configurations for battles',
    },
    {
        nmm: 'TOReborn Equipment Editor.nmm',
        id: 'equipment',
        name: 'Equipment Editor',
        desc: 'Edit weapon and armor properties',
    },
    {
        nmm: 'TOReborn Item Editor.nmm',
        id: 'items',
        name: 'Item Editor',
        desc: 'Edit consumable item properties',
    },
    {
        nmm: 'TOReborn Class Editor.nmm',
        id: 'classes',
        name: 'Class Editor',
        desc: 'Edit class stats, sprites, and properties',
    },
    {
        nmm: 'TOReborn Spell Editor.nmm',
        id: 'spells',
        name: 'Spell Editor',
        desc: 'Edit spell properties and effects',
    },
    {
        nmm: 'TOReborn Finisher Editor.nmm',
        id: 'finishers',
        name: 'Finisher Editor',
        desc: 'Edit finisher move properties',
    },
    {
        nmm: 'TOReborn Action Skill Editor.nmm',
        id: 'skills',
        name: 'Action Skill Editor',
        desc: 'Edit action skill properties',
    },
    {
        nmm: 'TOReborn Ease II Editor.nmm',
        id: 'ease2',
        name: 'Ease II Editor',
        desc: 'Edit Ease II spell properties',
    },
    {
        nmm: 'TOReborn Character Editor.nmm',
        id: 'characters',
        name: 'Character Editor',
        desc: 'Edit character stats and properties',
    },
    {
        nmm: 'TOReborn Skill Access Editor.nmm',
        id: 'skill_access',
        name: 'Skill Access Editor',
        desc: 'Edit which classes can learn each skill',
    },
    {
        nmm: 'TOReborn Spell Access Editor.nmm',
        id: 'spell_access',
        name: 'Spell Access Editor',
        desc: 'Edit which classes can use each spell',
    },
    {
        nmm: 'TOReborn Ease II Access Editor.nmm',
        id: 'ease2_access',
        name: 'Ease II Access Editor',
        desc: 'Edit Ease II spell class access',
    },
    {
        nmm: 'TOReborn Shop Editor.nmm',
        id: 'shop',
        name: 'Shop Editor',
        desc: 'Edit shop inventory and availability',
    }
];

function parseNmm(path) {
    const raw = readFileSync(path, 'utf-8');
    const lines = raw.split(/\r?\n/);
    let i = 0;
    while (i < lines.length && (lines[i].startsWith('#') || lines[i].trim() === '')) i++;

    const version = lines[i++];
    const moduleName = lines[i++];
    const baseOffset = lines[i++].trim();
    const entryCount = parseInt(lines[i++].trim());
    const entrySize = parseInt(lines[i++].trim());
    const entryNamesFile = lines[i++].trim();
    const unknown = lines[i++].trim();

    while (i < lines.length && lines[i].trim() === '') i++;

    const fields = [];
    while (i < lines.length) {
        while (i < lines.length && lines[i].trim() === '') i++;
        if (i >= lines.length) break;
        const name = lines[i++]?.trim();
        if (!name) break;
        const offset = parseInt(lines[i++]?.trim());
        const size = parseInt(lines[i++]?.trim());
        const typeCode = lines[i++]?.trim();
        const optionsFile = lines[i++]?.trim();
        if (isNaN(offset) || isNaN(size)) break;
        fields.push({ name, offset, size, typeCode, optionsFile });
    }

    return {
        baseOffset: parseInt(baseOffset, 16),
        entryCount,
        entrySize,
        entryNamesFile: entryNamesFile === 'NULL' ? null : entryNamesFile,
        fields
    };
}

const optionsCache = {};
function parseOptionsFile(filename) {
    if (optionsCache[filename]) return optionsCache[filename];
    const path = join(NMM_DIR, filename);
    if (!existsSync(path)) {
        console.warn(`  Warning: options file not found: ${filename}`);
        optionsCache[filename] = null;
        return null;
    }
    const raw = readFileSync(path, 'utf-8');
    const lines = raw.split(/\r?\n/).filter((l) => l.trim() !== '');
    const count = parseInt(lines[0]);
    if (isNaN(count)) {
        optionsCache[filename] = null;
        return null;
    }

    const options = [];
    for (let j = 1; j < lines.length; j++) {
        const line = lines[j].trim();
        if (!line) continue;
        const match = line.match(/^(0x[0-9A-Fa-f]+|\d+)\s+(.+)$/);
        if (match) {
            const value = match[1].startsWith('0x') ? parseInt(match[1], 16) : parseInt(match[1]);
            options.push({ value, label: match[2] });
        }
    }
    optionsCache[filename] = options;
    return options;
}

function parseEntryNamesFile(filename) {
    const path = join(NMM_DIR, filename);
    if (!existsSync(path)) {
        console.warn(`  Warning: entry names file not found: ${filename}`);
        return [];
    }
    const raw = readFileSync(path, 'utf-8');
    const lines = raw.split(/\r?\n/);
    const firstLine = lines[0]?.trim();
    if (/^\d+$/.test(firstLine)) {
        const names = [];
        for (let j = 1; j < lines.length; j++) {
            const line = lines[j].trim();
            if (!line) continue;
            const match = line.match(/^(?:0x[0-9A-Fa-f]+|\d+)\s+(.+)$/);
            if (match) names.push(match[1]);
        }
        return names;
    }
    return lines.filter((l) => l.trim() !== '').map((l) => l.trim());
}

function defaultEntryNames(entryCount) {
    return Array.from({ length: entryCount }, (_, index) => `Record ${index}`);
}

function escapeRustString(s) {
    return s.replace(/\\/g, '\\\\').replace(/"/g, '\\"');
}

function fieldTypeForNmm(typeCode, optionsFile) {
    if (typeCode === 'NEHU') return 'Hex';
    if (typeCode === 'NEDU') return 'Uint';
    if (typeCode === 'NDHU') {
        if (optionsFile === 'NULL' || !optionsFile) return 'Hex';
        if (optionsFile === 'Calc.txt') return 'Int';
        return 'Dropdown';
    }
    return 'Hex';
}

function toSnakeCase(filename) {
    return filename
        .replace(/\.txt$/, '')
        .replace(/[^a-zA-Z0-9]/g, '_')
        .replace(/_+/g, '_')
        .replace(/^_|_$/g, '')
        .toLowerCase();
}

function emitOptionsFn(fnName, opts) {
    let rust = `fn ${fnName}() -> Vec<FieldOption> {\n    vec![\n`;
    for (const opt of opts) {
        rust += `        FieldOption { value: ${opt.value}, label: "${escapeRustString(opt.label)}".to_string() },\n`;
    }
    rust += `    ]\n}\n\n`;
    return rust;
}

function generateModule(mod) {
    console.log(`Generating ${mod.id}...`);
    const nmm = parseNmm(join(NMM_DIR, mod.nmm));

    let entryNames = [];
    if (nmm.entryNamesFile) {
        entryNames = parseEntryNamesFile(nmm.entryNamesFile);
        if (entryNames.length > nmm.entryCount) {
            entryNames = entryNames.slice(0, nmm.entryCount);
        }
    } else {
        // Nightmare modules with NULL entry-name files fall back to numeric selectors.
        // We materialize those labels so the UI can still render a selector.
        entryNames = defaultEntryNames(nmm.entryCount);
    }

    // Collect unique option files used by this module's fields
    const usedOptionFiles = new Map(); // filename -> snake_case fn name
    for (const field of nmm.fields) {
        const ft = fieldTypeForNmm(field.typeCode, field.optionsFile);
        if (
            ft === 'Dropdown' &&
            field.optionsFile &&
            field.optionsFile !== 'NULL' &&
            field.optionsFile !== 'Calc.txt'
        ) {
            if (!usedOptionFiles.has(field.optionsFile)) {
                usedOptionFiles.set(field.optionsFile, `opt_${toSnakeCase(field.optionsFile)}`);
            }
        }
    }

    let rust = `use crate::modules::types::*;\n\n`;

    // Emit shared option functions at the top
    for (const [filename, fnName] of usedOptionFiles) {
        const opts = parseOptionsFile(filename);
        if (opts && opts.length > 0) {
            rust += emitOptionsFn(fnName, opts);
        }
    }

    rust += `pub fn definition() -> ModuleDefinition {\n`;
    rust += `    ModuleDefinition {\n`;
    rust += `        id: "${mod.id}".to_string(),\n`;
    rust += `        name: "${escapeRustString(mod.name)}".to_string(),\n`;
    rust += `        description: "${escapeRustString(mod.desc)}".to_string(),\n`;
    rust += `        base_offset: 0x${nmm.baseOffset.toString(16).toUpperCase()},\n`;
    rust += `        entry_count: ${nmm.entryCount},\n`;
    rust += `        entry_size: ${nmm.entrySize},\n`;

    rust += `        entry_names: vec![\n`;
    for (const name of entryNames) {
        rust += `            "${escapeRustString(name)}".to_string(),\n`;
    }
    rust += `        ],\n`;

    rust += `        fields: vec![\n`;

    for (const field of nmm.fields) {
        const ft = fieldTypeForNmm(field.typeCode, field.optionsFile);
        let optionsExpr = 'None';
        if (
            ft === 'Dropdown' &&
            field.optionsFile &&
            field.optionsFile !== 'NULL' &&
            field.optionsFile !== 'Calc.txt'
        ) {
            const fnName = usedOptionFiles.get(field.optionsFile);
            if (fnName) {
                optionsExpr = `Some(${fnName}())`;
            }
        }

        rust += `            FieldDefinition {\n`;
        rust += `                name: "${escapeRustString(field.name)}".to_string(),\n`;
        rust += `                offset: ${field.offset},\n`;
        rust += `                size: ${field.size},\n`;
        rust += `                field_type: FieldType::${ft},\n`;
        rust += `                options: ${optionsExpr},\n`;
        rust += `            },\n`;
    }

    rust += `        ],\n`;
    rust += `    }\n`;
    rust += `}\n`;

    // Tests
    rust += `\n#[cfg(test)]\nmod tests {\n    use super::*;\n\n`;
    rust += `    #[test]\n    fn metadata() {\n        let def = definition();\n`;
    rust += `        assert_eq!(def.id, "${mod.id}");\n`;
    rust += `        assert_eq!(def.base_offset, 0x${nmm.baseOffset.toString(16).toUpperCase()});\n`;
    rust += `        assert_eq!(def.entry_count, ${nmm.entryCount});\n`;
    rust += `        assert_eq!(def.entry_size, ${nmm.entrySize});\n    }\n\n`;
    rust += `    #[test]\n    fn field_count() {\n        assert_eq!(definition().fields.len(), ${nmm.fields.length});\n    }\n\n`;
    rust += `    #[test]\n    fn entry_names_count() {\n        let def = definition();\n`;
    rust += `        assert_eq!(def.entry_names.len(), ${entryNames.length});\n`;
    rust += `    }\n\n`;

    if (nmm.fields.length > 0) {
        const first = nmm.fields[0];
        rust += `    #[test]\n    fn first_field() {\n        let f = &definition().fields[0];\n`;
        rust += `        assert_eq!(f.name, "${escapeRustString(first.name)}");\n`;
        rust += `        assert_eq!(f.offset, ${first.offset});\n        assert_eq!(f.size, ${first.size});\n    }\n\n`;

        const last = nmm.fields[nmm.fields.length - 1];
        rust += `    #[test]\n    fn last_field() {\n        let f = &definition().fields[${nmm.fields.length - 1}];\n`;
        rust += `        assert_eq!(f.name, "${escapeRustString(last.name)}");\n`;
        rust += `        assert_eq!(f.offset, ${last.offset});\n        assert_eq!(f.size, ${last.size});\n    }\n`;
    }

    rust += `\n    #[test]\n    fn fields_within_entry_bounds() {\n        let def = definition();\n`;
    rust += `        for field in &def.fields {\n            assert!(\n`;
    rust += `                field.offset + field.size <= def.entry_size,\n`;
    rust += `                "field '{}' at offset {} + size {} exceeds entry_size {}",\n`;
    rust += `                field.name, field.offset, field.size, def.entry_size\n            );\n        }\n    }\n`;
    rust += `}\n`;

    const outPath = join(OUT_DIR, `${mod.id}.rs`);
    writeFileSync(outPath, rust);
    console.log(`  -> ${outPath} (${nmm.fields.length} fields, ${usedOptionFiles.size} shared option lists)`);
}

for (const mod of MODULES) {
    generateModule(mod);
}
console.log('Done!');
