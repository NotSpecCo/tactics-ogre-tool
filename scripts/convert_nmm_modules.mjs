// CLI wrapper around scripts/lib/nmm_convert.mjs.
// Converts reference_files/nightmare_modules_new into the JSON5 module set
// defined by docs/MODULE-SPEC.md.

import { existsSync, mkdirSync, readdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { convertTree } from './lib/nmm_convert.mjs';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const defaultSourceDir = path.join(repoRoot, 'reference_files/nightmare_modules_new');
const defaultOutDir = path.join(repoRoot, 'modules');

function parseArgs(argv) {
    const args = {
        sourceDir: defaultSourceDir,
        outDir: defaultOutDir,
        clean: false,
        dryRun: false,
        check: false
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
        } else if (arg === '--check') {
            args.check = true;
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
  --out <dir>     Output directory for generated JSON5.
  --clean         Delete the output directory before writing.
  --dry-run       Parse and report without writing files.
  --check         Re-convert and diff against the output directory;
                  exits nonzero on any difference (for CI).
`);
}

function walkJson5Files(dir) {
    const results = [];
    for (const entry of readdirSync(dir, { withFileTypes: true })) {
        const full = path.join(dir, entry.name);
        if (entry.isDirectory()) {
            results.push(...walkJson5Files(full));
        } else if (entry.isFile() && entry.name.toLowerCase().endsWith('.json5')) {
            results.push(full);
        }
    }
    return results.sort();
}

function toPosix(p) {
    return p.split(path.sep).join('/');
}

function runCheck(outputs, outDir) {
    const problems = [];

    for (const [rel, content] of [...outputs.entries()].sort(([a], [b]) => (a < b ? -1 : 1))) {
        const diskPath = path.join(outDir, rel);
        if (!existsSync(diskPath)) {
            problems.push(`missing: ${rel}`);
        } else if (readFileSync(diskPath, 'utf8') !== content) {
            problems.push(`differs: ${rel}`);
        }
    }

    if (existsSync(outDir)) {
        for (const diskPath of walkJson5Files(outDir)) {
            const rel = toPosix(path.relative(outDir, diskPath));
            if (!outputs.has(rel)) {
                problems.push(`stray: ${rel}`);
            }
        }
    }

    if (problems.length > 0) {
        for (const problem of problems) {
            console.error(problem);
        }
        console.error(`check failed: ${problems.length} problem(s)`);
        process.exit(1);
    }
    console.log(`check passed: ${outputs.size} files match ${outDir}`);
}

function main() {
    const args = parseArgs(process.argv.slice(2));
    if (!existsSync(args.sourceDir)) {
        throw new Error(`Source directory does not exist: ${args.sourceDir}`);
    }

    const { outputs, warnings, summary } = convertTree(args.sourceDir);

    for (const warning of warnings) {
        console.warn(`warning: ${warning}`);
    }

    if (args.check) {
        runCheck(outputs, args.outDir);
        return;
    }

    if (args.dryRun) {
        console.log(
            `Parsed ${summary.modules} modules, ${summary.options} option files, and ${summary.entries} entry files (dry run)`
        );
        return;
    }

    if (args.clean && existsSync(args.outDir)) {
        rmSync(args.outDir, { recursive: true, force: true });
    }

    for (const [rel, content] of outputs) {
        const outPath = path.join(args.outDir, rel);
        mkdirSync(path.dirname(outPath), { recursive: true });
        writeFileSync(outPath, content);
    }

    console.log(
        `Wrote ${summary.modules} modules, ${summary.options} option files, and ${summary.entries} entry files to ${args.outDir}`
    );
}

try {
    main();
} catch (error) {
    console.error(error instanceof Error ? error.message : error);
    process.exit(1);
}
