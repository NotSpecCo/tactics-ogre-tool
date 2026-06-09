import { existsSync, mkdirSync, readdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const defaultSourceDir = path.join(repoRoot, 'reference_files/nightmare_modules_new');
const defaultOutDir = path.join(repoRoot, 'modules');

const typeMap = {
    NEDU: { type: 'uint', display: 'decimal' },
    NEDS: { type: 'int' },
    NEHU: { type: 'uint', display: 'hex' },
    NDDU: { type: 'dropdown', display: 'decimal' },
    NDHU: { type: 'dropdown', display: 'hex' },
    HEXA: { type: 'bytes', display: 'hex' },
    TEXT: { type: 'text' }
};

function parseArgs(argv) {
    const args = {
        sourceDir: defaultSourceDir,
        outDir: defaultOutDir,
        clean: false,
        dryRun: false
    };

    for (let i = 0; i < argv.length; i += 1) {
        const arg = argv[i];
        if (arg === '--source') {
            args.sourceDir = path.resolve(argv[++i] ?? '');
        } else if (arg === '--out') {
            args.outDir = path.resolve(argv[++i] ?? '');
        } else if (arg === '--clean') {
            args.clean = true;
        } else if (arg === '--dry-run') {
            args.dryRun = true;
        } else if (arg === '--help' || arg === '-h') {
            printHelp();
            process.exit(0);
        } else {
            throw new Error(`Unknown argument: ${arg}`);
        }
    }

    return args;
}

function printHelp() {
    console.log(`Usage: npm run convert:nmm -- [options]

Options:
  --source <dir>  Source Nightmare module tree.
  --out <dir>     Output directory for generated YAML.
  --clean         Delete the output directory before writing.
  --dry-run       Parse and report without writing files.
`);
}

function walkFiles(dir, predicate) {
    const entries = readdirSync(dir, { withFileTypes: true });
    const files = [];

    for (const entry of entries) {
        const fullPath = path.join(dir, entry.name);
        if (entry.isDirectory()) {
            files.push(...walkFiles(fullPath, predicate));
        } else if (entry.isFile() && predicate(fullPath)) {
            files.push(fullPath);
        }
    }

    return files.sort((a, b) => a.localeCompare(b));
}

function logicalLines(filePath) {
    return readFileSync(filePath, 'utf8')
        .split(/\r?\n/)
        .map((text, index) => ({ text, line: index + 1, trim: text.trim() }))
        .filter((line) => line.trim !== '' && !line.trim.startsWith('#'));
}

function moduleNotes(filePath) {
    const lines = readFileSync(filePath, 'utf8').split(/\r?\n/);
    const notes = [];

    for (const text of lines) {
        const trim = text.trim();
        if (trim !== '' && !trim.startsWith('#')) {
            break;
        }
        if (!trim.startsWith('#')) {
            continue;
        }

        const comment = trim.slice(1).trim();
        if (comment === '' || comment === 'xlce' || /^0x[0-9a-f]+$/i.test(comment) || /^[0-9a-f]{8}$/i.test(comment)) {
            continue;
        }
        notes.push(comment);
    }

    return notes.join('\n');
}

function parseInteger(token, context) {
    const text = String(token).trim();
    if (/^0x[0-9a-f]+$/i.test(text)) {
        return Number.parseInt(text.slice(2), 16);
    }
    if (/^[0-9]+$/.test(text)) {
        return Number.parseInt(text, 10);
    }
    throw new Error(`${context}: invalid integer ${JSON.stringify(token)}`);
}

function parseValuePair(line, context) {
    const match = /^(0x[0-9a-f]+|[0-9]+)\s*(.*)$/i.exec(line.trim());
    if (!match) {
        throw new Error(`${context}: expected "value label", got ${JSON.stringify(line)}`);
    }

    return {
        value: parseInteger(match[1], context),
        valueWidth: match[1].toLowerCase().startsWith('0x') ? match[1].length - 2 : 0,
        label: match[2].trim()
    };
}

function parseNmm(filePath, sourceDir) {
    const lines = logicalLines(filePath);
    if (lines.length < 7) {
        throw new Error(`${filePath}: expected at least 7 logical header lines`);
    }

    const [fileCount, title, baseOffset, entryCount, entrySize, entryLabelsFile, characterTable] = lines
        .slice(0, 7)
        .map((line) => line.trim);

    if (fileCount !== '1') {
        throw new Error(`${filePath}: unsupported Nightmare file count ${fileCount}`);
    }

    if (characterTable !== 'NULL') {
        throw new Error(`${filePath}: character tables are not supported in schema version 1`);
    }

    const fieldLines = lines.slice(7);
    if (fieldLines.length % 5 !== 0) {
        throw new Error(`${filePath}: field list has ${fieldLines.length} logical lines`);
    }

    const fields = [];
    for (let i = 0; i < fieldLines.length; i += 5) {
        const [labelLine, offsetLine, sizeLine, typeCodeLine, optionsLine] = fieldLines.slice(i, i + 5);
        const typeCode = typeCodeLine.trim;
        if (!typeMap[typeCode]) {
            throw new Error(`${filePath}:${typeCodeLine.line}: unsupported field type ${typeCode}`);
        }

        fields.push({
            label: labelLine.trim,
            offset: parseInteger(offsetLine.trim, `${filePath}:${offsetLine.line}`),
            size: parseInteger(sizeLine.trim, `${filePath}:${sizeLine.line}`),
            typeCode,
            optionsPath: optionsLine.trim,
            sourceLine: labelLine.line
        });
    }

    return {
        sourcePath: toPosix(path.relative(sourceDir, filePath)),
        title,
        baseOffset: parseInteger(baseOffset, `${filePath}:base_offset`),
        entryCount: parseInteger(entryCount, `${filePath}:entry_count`),
        entrySize: parseInteger(entrySize, `${filePath}:entry_size`),
        entryLabelsPath: entryLabelsFile === 'NULL' ? null : entryLabelsFile,
        notes: moduleNotes(filePath),
        fields
    };
}

function toPosix(filePath) {
    return filePath.split(path.sep).join('/');
}

function toSnakeCase(value) {
    return value
        .replace(/([a-z0-9])([A-Z])/g, '$1_$2')
        .replace(/[^A-Za-z0-9]+/g, '_')
        .replace(/^_+|_+$/g, '')
        .replace(/_+/g, '_')
        .toLowerCase();
}

function uniqueId(base, usedIds) {
    let id = base;
    let suffix = 2;

    while (usedIds.has(id)) {
        id = `${base}_${suffix}`;
        suffix += 1;
    }

    usedIds.add(id);
    return id;
}

function labelFromTitle(title) {
    const parts = title.split('==>');
    return parts[parts.length - 1].trim();
}

function filesFromTitle(title) {
    if (title.includes('battle_data_release / pack')) {
        return ['battle/battle_data_release.dat'];
    }
    if (title.includes('menu_data / pack')) {
        return ['menu/menu_data.dat'];
    }
    if (title.includes('battle / entry / entry_unit_#### / pack')) {
        return ['battle/entry/entry_unit_*.dat'];
    }

    throw new Error(`Cannot map Nightmare title to target files: ${title}`);
}

function resolveNightmareSidecar(modulePath, sourceDir, sidecarPath) {
    if (sidecarPath === 'NULL') {
        return null;
    }

    const resolved = path.normalize(path.join(path.dirname(modulePath), sidecarPath));
    const relative = toPosix(path.relative(sourceDir, resolved));
    if (relative.startsWith('../')) {
        throw new Error(`${modulePath}: sidecar escapes source tree: ${sidecarPath}`);
    }

    return {
        absolutePath: resolved,
        relativePath: relative,
        parts: relative.split('/')
    };
}

function mappedSidecarPath(sidecar, kind) {
    if (!sidecar) {
        return null;
    }

    const [directory, filename] = sidecar.parts;
    const basename = toSnakeCase(filename.replace(/\.txt$/i, ''));

    if (kind === 'entry') {
        if (directory !== '_record') {
            throw new Error(`Expected _record sidecar, got ${sidecar.relativePath}`);
        }
        return `entries/${basename}.yml`;
    }

    if (directory === '_list') {
        return `options/${basename}.yml`;
    }
    if (directory === '_name') {
        return `options/name_${basename}.yml`;
    }

    throw new Error(`Expected _list or _name sidecar, got ${sidecar.relativePath}`);
}

function isSeparatorSidecar(sidecar) {
    return sidecar?.parts[0] === '_list' && sidecar.parts[1]?.toLowerCase() === 'separator.txt';
}

function convertModule(filePath, sourceDir) {
    const parsed = parseNmm(filePath, sourceDir);
    const id = toSnakeCase(parsed.sourcePath.replace(/\.nmm$/i, '').replace(/\//g, '_'));
    const label = labelFromTitle(parsed.title);
    const usedIds = new Set();
    let sectionOrdinal = 1;

    const fields = parsed.fields.map((field) => {
        const sidecar = resolveNightmareSidecar(filePath, sourceDir, field.optionsPath);
        const normalizedId = toSnakeCase(field.label);
        const fallbackId = isSeparatorSidecar(sidecar)
            ? `section_${sectionOrdinal}`
            : `field_${formatHex(field.offset, 2).toLowerCase().slice(2)}`;
        const baseId = /^[a-z]/.test(normalizedId) ? normalizedId : fallbackId;

        if (isSeparatorSidecar(sidecar)) {
            sectionOrdinal += 1;
            return {
                id: uniqueId(baseId, usedIds),
                label: field.label,
                type: 'section',
                sourceLine: field.sourceLine
            };
        }

        const mappedType = typeMap[field.typeCode];
        const converted = {
            id: uniqueId(baseId, usedIds),
            label: field.label,
            offset: field.offset,
            size: field.size,
            type: mappedType.type,
            sourceLine: field.sourceLine
        };

        if (mappedType.display && mappedType.display !== 'decimal') {
            converted.display = mappedType.display;
        }
        if (mappedType.type === 'dropdown') {
            converted.optionsFile = mappedSidecarPath(sidecar, 'option');
        }

        return converted;
    });

    markPartialOverlaps(fields);

    const entryLabelsSidecar = resolveNightmareSidecar(filePath, sourceDir, parsed.entryLabelsPath ?? 'NULL');
    const module = {
        schemaVersion: 1,
        id,
        label,
        description: `${label} table editor`,
        notes: parsed.notes,
        source: {
            format: 'nightmare',
            path: parsed.sourcePath,
            title: parsed.title
        },
        files: filesFromTitle(parsed.title),
        baseOffset: parsed.baseOffset,
        entry: {
            count: parsed.entryCount,
            size: parsed.entrySize,
            labelsFile: mappedSidecarPath(entryLabelsSidecar, 'entry')
        },
        fields
    };

    if (parsed.sourcePath === 'battle/entry/BattleUnit.nmm') {
        module.entry = {
            countFrom: {
                baseOffset: 0x20,
                offset: 0x04,
                size: 4,
                type: 'uint'
            },
            size: parsed.entrySize,
            labelsFile: null
        };
    }

    return module;
}

function markPartialOverlaps(fields) {
    const storedFields = fields.filter((field) => field.type !== 'section');

    for (let i = 0; i < storedFields.length; i += 1) {
        for (let j = i + 1; j < storedFields.length; j += 1) {
            const left = storedFields[i];
            const right = storedFields[j];
            const leftEnd = left.offset + left.size;
            const rightEnd = right.offset + right.size;
            const overlaps = left.offset < rightEnd && right.offset < leftEnd;
            const identical = left.offset === right.offset && left.size === right.size;

            if (overlaps && !identical) {
                left.overlap = true;
                right.overlap = true;
            }
        }
    }
}

function parseOptionFile(filePath) {
    const lines = logicalLines(filePath);
    if (lines.length === 0) {
        throw new Error(`${filePath}: empty option file`);
    }

    const declaredCount = parseInteger(lines[0].trim, `${filePath}:count`);
    const options = lines.slice(1).map((line) => parseValuePair(line.trim, `${filePath}:${line.line}`));

    return { declaredCount, options };
}

function parseRecordFile(filePath) {
    const lines = readFileSync(filePath, 'utf8')
        .split(/\r?\n/)
        .map((text) => text.trim())
        .filter((text) => text !== '' && !text.startsWith('#'));

    return lines.map((label, index) => ({
        value: index,
        label
    }));
}

function formatHex(value, minDigits) {
    const digits = Math.max(minDigits, value.toString(16).length);
    return `0x${value.toString(16).toUpperCase().padStart(digits, '0')}`;
}

function yamlScalar(value) {
    const text = String(value);
    if (text.includes("'")) {
        return JSON.stringify(text);
    }
    return `'${text}'`;
}

function yamlBlock(key, value, indent = '') {
    if (!value) {
        return '';
    }

    if (!value.includes('\n')) {
        return `${indent}${key}: ${yamlScalar(value)}\n`;
    }

    const lines = value.split('\n').map((line) => `${indent}  ${line}`);
    return `${indent}${key}: |-\n${lines.join('\n')}\n`;
}

function moduleToYaml(module) {
    let yaml = '';
    yaml += `schema_version: 1\n`;
    yaml += `id: ${module.id}\n`;
    yaml += `label: ${yamlScalar(module.label)}\n`;
    yaml += `description: ${yamlScalar(module.description)}\n`;
    yaml += yamlBlock('notes', module.notes);
    yaml += `\n`;
    yaml += `source:\n`;
    yaml += `  format: ${module.source.format}\n`;
    yaml += `  path: ${yamlScalar(module.source.path)}\n`;
    yaml += `  title: ${yamlScalar(module.source.title)}\n`;
    yaml += `\n`;
    yaml += `files:\n`;
    for (const file of module.files) {
        yaml += `  - ${yamlScalar(file)}\n`;
    }
    yaml += `\n`;
    yaml += `base_offset: ${formatHex(module.baseOffset, 8)}\n`;
    yaml += `endian: little\n`;
    yaml += `\n`;
    yaml += `entry:\n`;
    if (module.entry.countFrom) {
        yaml += `  count_from:\n`;
        yaml += `    base_offset: ${formatHex(module.entry.countFrom.baseOffset, 2)}\n`;
        yaml += `    offset: ${formatHex(module.entry.countFrom.offset, 2)}\n`;
        yaml += `    size: ${module.entry.countFrom.size}\n`;
        yaml += `    type: ${module.entry.countFrom.type}\n`;
    } else {
        yaml += `  count: ${module.entry.count}\n`;
    }
    yaml += `  size: ${formatHex(module.entry.size, 2)}\n`;
    yaml += `  labels_file: ${module.entry.labelsFile === null ? 'null' : yamlScalar(module.entry.labelsFile)}\n`;
    yaml += `\n`;
    yaml += `fields:\n`;
    module.fields.forEach((field, index) => {
        yaml += `  - id: ${field.id}\n`;
        yaml += `    label: ${yamlScalar(field.label)}\n`;
        yaml += `    type: ${field.type}\n`;
        if (field.type !== 'section') {
            yaml += `    offset: ${formatHex(field.offset, 2)}\n`;
            yaml += `    size: ${field.size}\n`;
            if (field.display) {
                yaml += `    display: ${field.display}\n`;
            }
            if (field.optionsFile) {
                yaml += `    options_file: ${yamlScalar(field.optionsFile)}\n`;
            }
            if (field.overlap) {
                yaml += `    overlap: true\n`;
            }
        }
        if (index < module.fields.length - 1) {
            yaml += `\n`;
        }
    });

    return yaml;
}

function listToYaml(items, minDigits) {
    let yaml = '';
    items.forEach((item, index) => {
        yaml += `- value: ${formatHex(item.value, minDigits)}\n`;
        yaml += `  label: ${yamlScalar(item.label)}\n`;
        if (index < items.length - 1) {
            yaml += `\n`;
        }
    });
    return yaml;
}

function writeFile(outDir, relativePath, contents, dryRun) {
    const outPath = path.join(outDir, relativePath);
    if (!dryRun) {
        mkdirSync(path.dirname(outPath), { recursive: true });
        writeFileSync(outPath, contents);
    }
}

function collectSidecars(sourceDir) {
    const options = new Map();
    const entries = new Map();

    for (const filePath of walkFiles(path.join(sourceDir, '_list'), (file) => file.endsWith('.txt'))) {
        if (path.basename(filePath).toLowerCase() === 'separator.txt') {
            continue;
        }
        const sidecar = {
            absolutePath: filePath,
            relativePath: toPosix(path.relative(sourceDir, filePath)),
            parts: toPosix(path.relative(sourceDir, filePath)).split('/')
        };
        options.set(mappedSidecarPath(sidecar, 'option'), filePath);
    }

    for (const filePath of walkFiles(path.join(sourceDir, '_name'), (file) => file.endsWith('.txt'))) {
        const sidecar = {
            absolutePath: filePath,
            relativePath: toPosix(path.relative(sourceDir, filePath)),
            parts: toPosix(path.relative(sourceDir, filePath)).split('/')
        };
        options.set(mappedSidecarPath(sidecar, 'option'), filePath);
    }

    for (const filePath of walkFiles(path.join(sourceDir, '_record'), (file) => file.endsWith('.txt'))) {
        const sidecar = {
            absolutePath: filePath,
            relativePath: toPosix(path.relative(sourceDir, filePath)),
            parts: toPosix(path.relative(sourceDir, filePath)).split('/')
        };
        entries.set(mappedSidecarPath(sidecar, 'entry'), filePath);
    }

    return { options, entries };
}

function validateModule(module, warnings) {
    for (const field of module.fields) {
        if (field.type === 'section') {
            continue;
        }

        if (field.offset + field.size > module.entry.size) {
            warnings.push(
                `${module.id}: ${field.id} exceeds entry size (${formatHex(field.offset, 2)} + ${field.size} > ${module.entry.size})`
            );
        }
    }
}

function main() {
    const args = parseArgs(process.argv.slice(2));
    if (!existsSync(args.sourceDir)) {
        throw new Error(`Source directory does not exist: ${args.sourceDir}`);
    }

    if (args.clean && !args.dryRun && existsSync(args.outDir)) {
        rmSync(args.outDir, { recursive: true, force: true });
    }

    const warnings = [];
    const nmmFiles = walkFiles(args.sourceDir, (file) => file.endsWith('.nmm'));
    const modules = nmmFiles.map((filePath) => convertModule(filePath, args.sourceDir));
    const { options, entries } = collectSidecars(args.sourceDir);

    for (const module of modules) {
        validateModule(module, warnings);
        writeFile(args.outDir, `${module.id}.yml`, moduleToYaml(module), args.dryRun);
    }

    for (const [relativePath, filePath] of options) {
        const parsed = parseOptionFile(filePath);
        if (parsed.declaredCount !== parsed.options.length) {
            warnings.push(
                `${toPosix(path.relative(args.sourceDir, filePath))}: declared ${parsed.declaredCount} options, parsed ${parsed.options.length}`
            );
        }

        const minDigits = Math.max(2, ...parsed.options.map((option) => option.valueWidth));
        writeFile(args.outDir, relativePath, listToYaml(parsed.options, minDigits), args.dryRun);
    }

    for (const [relativePath, filePath] of entries) {
        const parsed = parseRecordFile(filePath);
        const maxValue = Math.max(0, parsed.length - 1);
        const minDigits = Math.max(2, maxValue.toString(16).length);
        writeFile(args.outDir, relativePath, listToYaml(parsed, minDigits), args.dryRun);
    }

    for (const warning of warnings) {
        console.warn(`Warning: ${warning}`);
    }

    const action = args.dryRun ? 'Parsed' : 'Wrote';
    console.log(
        `${action} ${modules.length} modules, ${options.size} option files, and ${entries.size} entry files to ${args.outDir}`
    );
}

try {
    main();
} catch (error) {
    console.error(error instanceof Error ? error.message : error);
    process.exit(1);
}
