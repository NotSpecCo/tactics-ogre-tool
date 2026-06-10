// Conversion library for Nightmare modules -> JSON5 module format.
// Implements the "Converting Nightmare Modules" rules in docs/MODULE-SPEC.md.
// Pure text-level functions plus a filesystem orchestrator (convertTree).

import { readdirSync, readFileSync } from 'node:fs';
import path from 'node:path';

export const TYPE_CODE_MAP = {
    NEDU: { type: 'uint', display: 'decimal' },
    NEDS: { type: 'int' },
    NEHU: { type: 'uint', display: 'hex' },
    NDDU: { type: 'dropdown', display: 'decimal' },
    NDHU: { type: 'dropdown', display: 'hex' },
    HEXA: { type: 'bytes' },
    TEXT: { type: 'text' }
};

const SIDECAR_DIRS = ['_list', '_name', '_record'];

// Deliberate partial views: each shares its block with the corresponding
// full-table module and exposes only entry 0, so its declared count is
// intentionally smaller than the block header count. These convert as
// fixed-count tables (spec: "Header-Driven Table Detection").
export const PARTIAL_VIEW_MODULES = new Set([
    'battle/HealthDamageConfiguration.nmm',
    'battle/HealthRestoreConfiguration.nmm',
    'battle/HitRateConfiguration.nmm'
]);

const BLOCK_HEADER_SIZE = 0x10;

// --- line handling ---------------------------------------------------------

// Split on \n, \r\n, or \r (mixed allowed) and trim trailing whitespace from
// each line, per the spec's .nmm Parsing Rules.
export function splitSourceLines(text) {
    return text.split(/\r\n|\r|\n/).map((raw, index) => {
        const trimmedEnd = raw.replace(/\s+$/u, '');
        const trimmed = trimmedEnd.trim();
        let kind = 'content';
        if (trimmed === '') {
            kind = 'blank';
        } else if (trimmed.startsWith('#')) {
            kind = 'comment';
        }
        return { line: index + 1, text: trimmedEnd, trimmed, kind };
    });
}

function joinComments(commentLines) {
    return commentLines.map((c) => c.trimmed.replace(/^#/, '').trim()).join('\n');
}

// --- name normalization ----------------------------------------------------

export function normalizeName(value) {
    return String(value)
        .replace(/([a-z])([A-Z])/g, '$1_$2')
        .replace(/([A-Z])([A-Z][a-z])/g, '$1_$2')
        .toLowerCase()
        .replace(/[^a-z0-9]+/g, '_')
        .replace(/^_+|_+$/g, '');
}

export function moduleIdFromSourcePath(relPath) {
    return relPath
        .replace(/\.nmm$/i, '')
        .split('/')
        .map(normalizeName)
        .join('_');
}

// --- title handling --------------------------------------------------------

export function labelFromTitle(title) {
    const parts = title.split('==>');
    return parts[parts.length - 1].trim();
}

export function filesFromTitle(title) {
    if (title.includes('battle_data_release / pack')) {
        return ['battle/battle_data_release.dat'];
    }
    if (title.includes('menu_data / pack')) {
        return ['menu/menu_data.dat'];
    }
    if (title.includes('battle / entry / entry_unit_#### / pack')) {
        return ['battle/entry/entry_unit_*.dat'];
    }
    throw new Error(`cannot map Nightmare title to target files: ${title}`);
}

// --- integers ---------------------------------------------------------------

export function parseIntegerToken(token, context) {
    const text = String(token).trim();
    if (/^0x[0-9a-f]+$/i.test(text)) {
        return Number.parseInt(text.slice(2), 16);
    }
    if (/^[0-9]+$/.test(text)) {
        return Number.parseInt(text, 10);
    }
    throw new Error(`${context}: invalid integer ${JSON.stringify(token)}`);
}

// --- .nmm parsing ------------------------------------------------------------

// A maximal run of consecutive comment lines is treated as one or more disabled
// field blocks (and skipped silently) when it consists of whole 5-line groups
// whose 2nd/3rd lines are integers and whose 4th line is a known type code.
function isDisabledFieldRun(run) {
    if (run.length === 0 || run.length % 5 !== 0) {
        return false;
    }
    for (let i = 0; i < run.length; i += 5) {
        const stripped = run.slice(i, i + 5).map((c) => c.trimmed.replace(/^#/, '').trim());
        const [, offset, size, typeCode] = stripped;
        if (!/^(0x[0-9a-f]+|[0-9]+)$/i.test(offset) || !/^(0x[0-9a-f]+|[0-9]+)$/i.test(size)) {
            return false;
        }
        if (!Object.hasOwn(TYPE_CODE_MAP, typeCode)) {
            return false;
        }
    }
    return true;
}

export function parseNmmText(text, context) {
    const warnings = [];
    const lines = splitSourceLines(text);
    const content = lines.filter((l) => l.kind === 'content');
    const comments = lines.filter((l) => l.kind === 'comment');

    if (content.length < 7) {
        throw new Error(`${context}: expected at least 7 logical header lines, found ${content.length}`);
    }

    // The first three comment lines are Nightmare file-verification metadata:
    // checksum, ID-string address, ID string (content varies; some files have
    // a bare '#' checksum line). The address and ID string drive header-driven
    // table detection per the spec's "Header-Driven Table Detection".
    const preHeader = comments.filter((c) => c.line < content[0].line);
    if (preHeader.length < 3) {
        warnings.push(`${context}: expected 3 metadata comment lines before the header, found ${preHeader.length}`);
    }
    const metaTokens = preHeader.slice(0, 3).map((c) => c.trimmed.replace(/^#/, '').trim());
    let metaAddress = null;
    if (preHeader.length >= 2 && /^(0x[0-9a-f]+|[0-9]+)$/i.test(metaTokens[1])) {
        metaAddress = parseIntegerToken(metaTokens[1], `${context}:${preHeader[1].line}`);
    }
    const metaId = preHeader.length >= 3 ? metaTokens[2] : null;
    const noteComments = preHeader.slice(3);
    const notes = joinComments(noteComments);

    const header = content.slice(0, 7);
    const fileCount = header[0].trimmed;
    if (fileCount !== '1') {
        throw new Error(`${context}:${header[0].line}: unsupported Nightmare file count ${JSON.stringify(fileCount)}`);
    }

    const title = header[1].trimmed;
    const baseOffset = parseIntegerToken(header[2].trimmed, `${context}:${header[2].line}`);
    const entryCount = parseIntegerToken(header[3].trimmed, `${context}:${header[3].line}`);
    const entrySize = parseIntegerToken(header[4].trimmed, `${context}:${header[4].line}`);
    const labelsPath = header[5].trimmed === 'NULL' ? null : header[5].trimmed;
    // header[6] is the Nightmare character table path, ignored in version 1.

    const fieldContent = content.slice(7);
    if (fieldContent.length % 5 !== 0) {
        throw new Error(`${context}: field list has ${fieldContent.length} logical lines, not a multiple of 5`);
    }

    const consumed = new Set(preHeader.map((c) => c.line));
    const fields = [];
    for (let i = 0; i < fieldContent.length; i += 5) {
        const [labelLine, offsetLine, sizeLine, typeLine, optionsLine] = fieldContent.slice(i, i + 5);

        // A run of comment lines directly above the label (no blank line
        // between) is that field's documentation, unless it is a disabled
        // field block (spec comment rules 3 and 4).
        const above = [];
        for (let lineIdx = labelLine.line - 2; lineIdx >= 0; lineIdx -= 1) {
            if (lines[lineIdx].kind !== 'comment') {
                break;
            }
            above.unshift(lines[lineIdx]);
        }
        const aboveNotes = above.length > 0 && !isDisabledFieldRun(above) ? above : [];

        const inner = comments.filter((c) => c.line > labelLine.line && c.line < optionsLine.line);
        const noteLines = [...aboveNotes, ...inner];
        for (const c of noteLines) {
            consumed.add(c.line);
        }
        fields.push({
            label: labelLine.trimmed,
            notes: joinComments(noteLines),
            offset: parseIntegerToken(offsetLine.trimmed, `${context}:${offsetLine.line}`),
            size: parseIntegerToken(sizeLine.trimmed, `${context}:${sizeLine.line}`),
            typeCode: typeLine.trimmed,
            optionsPath: optionsLine.trimmed,
            line: labelLine.line
        });
    }

    // Remaining comments between field blocks are not converted (spec comment
    // rule 5). Runs that do not look like disabled field blocks are surfaced
    // for human review.
    const dropped = comments.filter((c) => !consumed.has(c.line));
    let run = [];
    const flushRun = () => {
        if (run.length > 0 && !isDisabledFieldRun(run)) {
            const preview = run[0].trimmed.replace(/^#/, '').trim();
            warnings.push(
                `${context}:${run[0].line}: dropped ${run.length} unconverted comment line(s) between fields: ${JSON.stringify(preview)}`
            );
        }
        run = [];
    };
    for (const c of dropped) {
        if (run.length > 0 && c.line !== run[run.length - 1].line + 1) {
            flushRun();
        }
        run.push(c);
    }
    flushRun();

    return { title, baseOffset, entryCount, entrySize, labelsPath, notes, fields, warnings, metaAddress, metaId };
}

// --- sidecar references ------------------------------------------------------

// Resolves a sidecar token from a .nmm file to its location in the source tree.
// Returns null for 'NULL'.
export function resolveSidecarRef(moduleRelPath, token) {
    if (!token || token === 'NULL') {
        return null;
    }
    const dir = path.posix.dirname(moduleRelPath);
    const resolved = path.posix.normalize(path.posix.join(dir, token.split('\\').join('/')));
    if (resolved === '..' || resolved.startsWith('../')) {
        throw new Error(`${moduleRelPath}: sidecar escapes source tree: ${token}`);
    }
    const parts = resolved.split('/');
    if (parts.length !== 2 || !SIDECAR_DIRS.includes(parts[0])) {
        throw new Error(`${moduleRelPath}: unexpected sidecar location: ${resolved}`);
    }
    const [kind, filename] = parts;
    return { kind, filename, basename: filename.replace(/\.txt$/i, ''), sourcePath: resolved };
}

export function isSeparatorRef(ref) {
    return ref !== null && ref.kind === '_list' && ref.filename.toLowerCase() === 'separator.txt';
}

export function mappedOptionsPath(ref) {
    const basename = normalizeName(ref.basename);
    if (ref.kind === '_list') {
        return `options/${basename}.json5`;
    }
    if (ref.kind === '_name') {
        return `options/name_${basename}.json5`;
    }
    throw new Error(`expected a _list or _name sidecar, got ${ref.sourcePath}`);
}

export function mappedEntriesPath(ref) {
    if (ref.kind !== '_record') {
        throw new Error(`expected a _record sidecar, got ${ref.sourcePath}`);
    }
    return `entries/${normalizeName(ref.basename)}.json5`;
}

// --- module conversion -------------------------------------------------------

export function convertNmmText(text, relPath) {
    const parsed = parseNmmText(text, relPath);
    const warnings = [...parsed.warnings];

    let sectionOrdinal = 0;
    const drafts = parsed.fields.map((f) => {
        const ref = resolveSidecarRef(relPath, f.optionsPath);
        const normalized = normalizeName(f.label);

        if (isSeparatorRef(ref)) {
            sectionOrdinal += 1;
            const baseId = /^[a-z]/.test(normalized) ? normalized : `section_${sectionOrdinal}`;
            const field = { label: f.label, type: 'section' };
            if (f.notes) {
                field.notes = f.notes;
            }
            return { baseId, field };
        }

        const mapped = TYPE_CODE_MAP[f.typeCode];
        if (!mapped) {
            throw new Error(`${relPath}:${f.line}: unsupported field type code ${JSON.stringify(f.typeCode)}`);
        }

        const baseId = /^[a-z]/.test(normalized) ? normalized : `field_0x${f.offset.toString(16).padStart(2, '0')}`;

        const field = { label: f.label, offset: f.offset, size: f.size, type: mapped.type };
        if (f.notes) {
            field.notes = f.notes;
        }
        if (mapped.display === 'hex') {
            field.display = 'hex';
        }
        if (mapped.type === 'dropdown') {
            if (ref === null) {
                throw new Error(`${relPath}:${f.line}: dropdown field ${JSON.stringify(f.label)} has no options file`);
            }
            field.options_file = mappedOptionsPath(ref);
        } else if (ref !== null) {
            throw new Error(
                `${relPath}:${f.line}: non-dropdown field ${JSON.stringify(f.label)} references options file ${f.optionsPath}`
            );
        }
        return { baseId, field };
    });

    // Deduplication runs after all IDs (including fallbacks) are generated:
    // later occurrences get _2, _3, ... in field order.
    const used = new Set();
    const fields = drafts.map((d) => {
        let id = d.baseId;
        let suffix = 2;
        while (used.has(id)) {
            id = `${d.baseId}_${suffix}`;
            suffix += 1;
        }
        used.add(id);
        return { id, ...d.field };
    });

    const files = filesFromTitle(parsed.title);
    const labelsRef = resolveSidecarRef(relPath, parsed.labelsPath ?? 'NULL');
    const labelsFile = labelsRef === null ? null : mappedEntriesPath(labelsRef);

    // Header-driven table detection: the metadata ID string is 'xlce' and the
    // metadata address is the block header at base_offset - 0x10, except for
    // the deliberate partial views.
    const headerDriven =
        parsed.metaId === 'xlce' &&
        parsed.metaAddress === parsed.baseOffset - BLOCK_HEADER_SIZE &&
        !PARTIAL_VIEW_MODULES.has(relPath);

    let entry;
    if (headerDriven) {
        // Literal targets keep the declared count as the expected count; glob
        // targets drop it, since per-file counts vary by design.
        const isGlob = files.some((f) => /[*?]/.test(f));
        entry = {
            header: true,
            ...(isGlob ? {} : { count: parsed.entryCount }),
            size: parsed.entrySize,
            labels_file: labelsFile
        };
    } else {
        entry = {
            count: parsed.entryCount,
            size: parsed.entrySize,
            labels_file: labelsFile
        };
    }

    const module = {
        schema_version: 1,
        id: moduleIdFromSourcePath(relPath),
        label: labelFromTitle(parsed.title),
        notes: parsed.notes,
        source: { format: 'nightmare', path: relPath, title: parsed.title },
        files,
        base_offset: parsed.baseOffset,
        endian: 'little',
        entry,
        fields
    };

    return { module, warnings };
}

// --- sidecar parsing ----------------------------------------------------------

function logicalLines(text) {
    return splitSourceLines(text).filter((l) => l.kind === 'content');
}

export function parseListText(text, context) {
    const warnings = [];
    const lines = logicalLines(text);
    if (lines.length === 0) {
        throw new Error(`${context}: empty list file`);
    }

    const declaredCount = parseIntegerToken(lines[0].trimmed, `${context}:${lines[0].line}`);
    const items = lines.slice(1).map((l) => {
        const match = /^(0x[0-9a-f]+|[0-9]+)(?:\s+(.*))?$/i.exec(l.trimmed);
        if (!match) {
            throw new Error(`${context}:${l.line}: expected "value label", got ${JSON.stringify(l.trimmed)}`);
        }
        const token = match[1];
        const label = (match[2] ?? '').trim();
        return {
            value: parseIntegerToken(token, `${context}:${l.line}`),
            width: token.toLowerCase().startsWith('0x') ? token.length - 2 : 0,
            label: label === '' ? 'unknown' : label
        };
    });

    if (declaredCount !== items.length) {
        warnings.push(`${context}: declared ${declaredCount} items, parsed ${items.length}`);
    }

    return { declaredCount, items, warnings };
}

export function parseRecordText(text) {
    return logicalLines(text).map((l, index) => ({ value: index, width: 0, label: l.trimmed }));
}

// --- serialization -------------------------------------------------------------

export function formatHex(value, minDigits) {
    return `0x${value.toString(16).padStart(minDigits, '0')}`;
}

function q(value) {
    const escaped = String(value)
        .replace(/\\/g, '\\\\')
        .replace(/'/g, "\\'")
        .replace(/\n/g, '\\n')
        .replace(/\r/g, '\\r')
        .replace(/\t/g, '\\t')
        // eslint-disable-next-line no-control-regex
        .replace(/[\u0000-\u001f]/g, (ch) => `\\x${ch.charCodeAt(0).toString(16).padStart(2, '0')}`);
    return `'${escaped}'`;
}

export function serializeModule(module) {
    const out = [];
    out.push('{');
    out.push(`    schema_version: ${module.schema_version},`);
    out.push(`    id: ${q(module.id)},`);
    out.push(`    label: ${q(module.label)},`);
    if (module.notes) {
        out.push(`    notes: ${q(module.notes)},`);
    }
    out.push('');
    out.push('    source: {');
    out.push(`        format: ${q(module.source.format)},`);
    out.push(`        path: ${q(module.source.path)},`);
    out.push(`        title: ${q(module.source.title)}`);
    out.push('    },');
    out.push('');
    out.push(`    files: [${module.files.map(q).join(', ')}],`);
    out.push('');
    out.push(`    base_offset: ${formatHex(module.base_offset, 8)},`);
    out.push(`    endian: ${q(module.endian)},`);
    out.push('');
    out.push('    entry: {');
    if (module.entry.header) {
        out.push('        header: true,');
    }
    if (module.entry.count !== undefined) {
        out.push(`        count: ${module.entry.count},`);
    }
    out.push(`        size: ${module.entry.size},`);
    out.push(`        labels_file: ${module.entry.labels_file === null ? 'null' : q(module.entry.labels_file)}`);
    out.push('    },');
    out.push('');
    out.push('    fields: [');
    module.fields.forEach((field, index) => {
        const props = [`id: ${q(field.id)}`, `label: ${q(field.label)}`];
        if (field.notes) {
            props.push(`notes: ${q(field.notes)}`);
        }
        if (field.type !== 'section') {
            props.push(`offset: ${formatHex(field.offset, 2)}`);
            props.push(`size: ${field.size}`);
        }
        props.push(`type: ${q(field.type)}`);
        if (field.display) {
            props.push(`display: ${q(field.display)}`);
        }
        if (field.options_file) {
            props.push(`options_file: ${q(field.options_file)}`);
        }
        out.push('        {');
        props.forEach((prop, i) => {
            out.push(`            ${prop}${i < props.length - 1 ? ',' : ''}`);
        });
        out.push(`        }${index < module.fields.length - 1 ? ',' : ''}`);
    });
    out.push('    ]');
    out.push('}');
    return `${out.join('\n')}\n`;
}

export function serializeItems(items, hexDigits) {
    const out = ['['];
    items.forEach((item, index) => {
        const comma = index < items.length - 1 ? ',' : '';
        out.push(`    { value: ${formatHex(item.value, hexDigits)}, label: ${q(item.label)} }${comma}`);
    });
    out.push(']');
    return `${out.join('\n')}\n`;
}

// --- tree conversion -------------------------------------------------------------

function toPosix(p) {
    return p.split(path.sep).join('/');
}

function walkFiles(dir, predicate) {
    const results = [];
    for (const entry of readdirSync(dir, { withFileTypes: true })) {
        const full = path.join(dir, entry.name);
        if (entry.isDirectory()) {
            results.push(...walkFiles(full, predicate));
        } else if (entry.isFile() && predicate(full)) {
            results.push(full);
        }
    }
    return results.sort();
}

function sidecarSources(sourceDir, sub) {
    const dir = path.join(sourceDir, sub);
    return walkFiles(dir, (f) => f.toLowerCase().endsWith('.txt')).map((f) => ({
        absolute: f,
        relative: toPosix(path.relative(sourceDir, f)),
        basename: path.basename(f).replace(/\.txt$/i, '')
    }));
}

export function convertTree(sourceDir) {
    const warnings = [];
    const outputs = new Map();
    const outputSources = new Map();

    const addOutput = (rel, content, source) => {
        if (outputs.has(rel)) {
            throw new Error(`output collision: ${rel} generated from both ${outputSources.get(rel)} and ${source}`);
        }
        outputs.set(rel, content);
        outputSources.set(rel, source);
    };

    const nmmPaths = walkFiles(sourceDir, (f) => f.toLowerCase().endsWith('.nmm')).map((f) =>
        toPosix(path.relative(sourceDir, f))
    );

    const modules = [];
    for (const rel of nmmPaths) {
        const text = readFileSync(path.join(sourceDir, rel), 'utf8');
        const { module, warnings: moduleWarnings } = convertNmmText(text, rel);
        warnings.push(...moduleWarnings);
        modules.push(module);
        addOutput(`${module.id}.json5`, serializeModule(module), rel);
    }

    let optionCount = 0;
    for (const source of sidecarSources(sourceDir, '_list')) {
        if (path.basename(source.relative).toLowerCase() === 'separator.txt') {
            continue;
        }
        const { items, warnings: listWarnings } = parseListText(readFileSync(source.absolute, 'utf8'), source.relative);
        warnings.push(...listWarnings);
        const hexDigits = Math.max(2, ...items.map((i) => i.width));
        addOutput(`options/${normalizeName(source.basename)}.json5`, serializeItems(items, hexDigits), source.relative);
        optionCount += 1;
    }
    for (const source of sidecarSources(sourceDir, '_name')) {
        const { items, warnings: listWarnings } = parseListText(readFileSync(source.absolute, 'utf8'), source.relative);
        warnings.push(...listWarnings);
        const hexDigits = Math.max(2, ...items.map((i) => i.width));
        addOutput(
            `options/name_${normalizeName(source.basename)}.json5`,
            serializeItems(items, hexDigits),
            source.relative
        );
        optionCount += 1;
    }

    let entryCount = 0;
    for (const source of sidecarSources(sourceDir, '_record')) {
        const items = parseRecordText(readFileSync(source.absolute, 'utf8'));
        const hexDigits = Math.max(2, Math.max(0, items.length - 1).toString(16).length);
        addOutput(`entries/${normalizeName(source.basename)}.json5`, serializeItems(items, hexDigits), source.relative);
        entryCount += 1;
    }

    // Every sidecar path referenced by a module must have been generated.
    for (const module of modules) {
        const refs = [module.entry.labels_file, ...module.fields.map((f) => f.options_file)].filter(
            (r) => r !== null && r !== undefined
        );
        for (const ref of refs) {
            if (!outputs.has(ref)) {
                throw new Error(`${module.id}: references sidecar ${ref}, which was not generated`);
            }
        }
    }

    return {
        outputs,
        modules,
        warnings,
        summary: { modules: modules.length, options: optionCount, entries: entryCount }
    };
}
