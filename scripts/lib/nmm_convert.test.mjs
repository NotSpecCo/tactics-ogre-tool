import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { describe, expect, it } from 'vitest';
import {
    convertNmmText,
    convertTree,
    filesFromTitle,
    formatHex,
    isSeparatorRef,
    labelFromTitle,
    mappedEntriesPath,
    mappedOptionsPath,
    moduleIdFromSourcePath,
    normalizeName,
    parseIntegerToken,
    parseListText,
    parseNmmText,
    parseRecordText,
    resolveSidecarRef,
    serializeItems,
    serializeModule,
    splitSourceLines
} from './nmm_convert.mjs';

const referenceDir = path.resolve(
    path.dirname(fileURLToPath(import.meta.url)),
    '../../reference_files/nightmare_modules_new'
);

// Builds a minimal valid .nmm text. Fields are arrays of 5 logical lines.
function nmm({
    metadata = ['#181ADE14', '#0x003BA490', '#xlce'],
    prose = [],
    title = 'Tactics Ogre: Reborn ==> battle_data_release / pack ==> Test',
    baseOffset = '0x00000100',
    count = '0x00000010',
    size = '0x00000020',
    labels = 'NULL',
    fields = []
} = {}) {
    const lines = [...metadata, '', ...prose, '', '1', title, baseOffset, count, size, labels, 'NULL', ''];
    for (const field of fields) {
        lines.push(...field, '');
    }
    return lines.join('\n');
}

describe('splitSourceLines', () => {
    it('splits on \\n, \\r\\n, and \\r, including mixed conventions', () => {
        const lines = splitSourceLines('one\r\ntwo\rthree\nfour');
        expect(lines.map((l) => l.trimmed)).toEqual(['one', 'two', 'three', 'four']);
    });

    it('trims trailing whitespace so labels carry no stray \\r or spaces', () => {
        const lines = splitSourceLines('label \t\nnext');
        expect(lines[0].text).toBe('label');
    });

    it('classifies blank, comment, and content lines', () => {
        const lines = splitSourceLines('#comment\n\n  \nvalue');
        expect(lines.map((l) => l.kind)).toEqual(['comment', 'blank', 'blank', 'content']);
    });
});

describe('normalizeName', () => {
    it('matches the spec examples', () => {
        expect(normalizeName('ItemType')).toBe('item_type');
        expect(normalizeName('BattleUnitHeader')).toBe('battle_unit_header');
        expect(normalizeName('Skill: <not used> 0006')).toBe('skill_not_used_0006');
        expect(normalizeName('? Flag ?')).toBe('flag');
        expect(normalizeName('??')).toBe('');
    });

    it('splits uppercase runs followed by lowercase', () => {
        expect(normalizeName('XLCE')).toBe('xlce');
        expect(normalizeName('ABCDef')).toBe('abc_def');
    });

    it('does not split between digits and uppercase letters', () => {
        expect(normalizeName('EffectHitRate1')).toBe('effect_hit_rate1');
        expect(normalizeName('Record Size = C4')).toBe('record_size_c4');
    });
});

describe('moduleIdFromSourcePath', () => {
    it('normalizes each segment and joins with underscores', () => {
        expect(moduleIdFromSourcePath('battle/Class.nmm')).toBe('battle_class');
        expect(moduleIdFromSourcePath('battle/entry/BattleUnit.nmm')).toBe('battle_entry_battle_unit');
        expect(moduleIdFromSourcePath('menu/Shop.nmm')).toBe('menu_shop');
    });
});

describe('labelFromTitle', () => {
    it('takes the final segment after the last ==>, trimmed', () => {
        expect(labelFromTitle('A ==> B ==> Armament (Equipment)')).toBe('Armament (Equipment)');
        expect(labelFromTitle('No arrows at all')).toBe('No arrows at all');
    });
});

describe('filesFromTitle', () => {
    it('maps the three known title segments', () => {
        expect(filesFromTitle('X ==> battle_data_release / pack ==> Y')).toEqual(['battle/battle_data_release.dat']);
        expect(filesFromTitle('X ==> menu_data / pack ==> Y')).toEqual(['menu/menu_data.dat']);
        expect(filesFromTitle('X ==> battle / entry / entry_unit_#### / pack ==> Y')).toEqual([
            'battle/entry/entry_unit_*.dat'
        ]);
    });

    it('throws on unknown titles', () => {
        expect(() => filesFromTitle('X ==> something_else / pack ==> Y')).toThrow(/cannot map/);
    });
});

describe('parseIntegerToken', () => {
    it('accepts decimal and case-insensitive hex', () => {
        expect(parseIntegerToken('23', 't')).toBe(23);
        expect(parseIntegerToken('0x00000010', 't')).toBe(16);
        expect(parseIntegerToken('0X0A', 't')).toBe(10);
    });

    it('rejects anything else', () => {
        for (const bad of ['-1', '+1', '1.5', 'abc', '0x', '']) {
            expect(() => parseIntegerToken(bad, 't')).toThrow(/invalid integer/);
        }
    });
});

describe('parseNmmText', () => {
    it('parses the header, skipping the three metadata comments by position', () => {
        const parsed = parseNmmText(nmm({ metadata: ['#', '#0x00000020', '#xlce'] }), 'test.nmm');
        expect(parsed.title).toContain('battle_data_release');
        expect(parsed.baseOffset).toBe(0x100);
        expect(parsed.entryCount).toBe(16);
        expect(parsed.entrySize).toBe(32);
        expect(parsed.labelsPath).toBeNull();
        expect(parsed.notes).toBe('');
    });

    it('accepts decimal header numerics', () => {
        const parsed = parseNmmText(nmm({ count: '23' }), 'test.nmm');
        expect(parsed.entryCount).toBe(23);
    });

    it('converts prose comments before the header to module notes, joined by newlines', () => {
        const parsed = parseNmmText(nmm({ prose: ['#Line one.', '#Line two.'] }), 'test.nmm');
        expect(parsed.notes).toBe('Line one.\nLine two.');
    });

    it('converts comments inside a field block to that field notes', () => {
        const parsed = parseNmmText(
            nmm({ fields: [['Power', '#How hard it hits.', '#Really.', '0x00', '2', 'NEDU', 'NULL']] }),
            'test.nmm'
        );
        expect(parsed.fields).toHaveLength(1);
        expect(parsed.fields[0].notes).toBe('How hard it hits.\nReally.');
    });

    it('converts a comment run directly above a field label to that field notes', () => {
        const text = nmm({
            fields: [
                ['Alpha', '0x00', '1', 'NEDU', 'NULL'],
                ['#Describes Beta.', '#In two lines.', 'Beta', '0x01', '1', 'NEDU', 'NULL']
            ]
        });
        const parsed = parseNmmText(text, 'test.nmm');
        expect(parsed.fields[1].notes).toBe('Describes Beta.\nIn two lines.');
        expect(parsed.warnings).toEqual([]);
    });

    it('joins above-label and after-label comments in file order', () => {
        const text = nmm({
            fields: [['#Above note.', 'Gamma', '#After note.', '0x00', '1', 'NEDU', 'NULL']]
        });
        const parsed = parseNmmText(text, 'test.nmm');
        expect(parsed.fields[0].notes).toBe('Above note.\nAfter note.');
    });

    it('drops comment runs separated from the next label by a blank line, with a warning', () => {
        const text = nmm({
            fields: [
                ['Alpha', '0x00', '1', 'NEDU', 'NULL'],
                ['#Scratch note about nothing.', '', 'Beta', '0x01', '1', 'NEDU', 'NULL']
            ]
        });
        const parsed = parseNmmText(text, 'test.nmm');
        expect(parsed.fields.map((f) => f.label)).toEqual(['Alpha', 'Beta']);
        expect(parsed.fields[1].notes).toBe('');
        expect(parsed.warnings).toHaveLength(1);
        expect(parsed.warnings[0]).toContain('Scratch note about nothing.');
    });

    it('never converts disabled field blocks to notes, even directly above a label', () => {
        const text = nmm({
            fields: [
                ['Alpha', '0x00', '1', 'NEDU', 'NULL'],
                ['#Disabled Field', '#0x14', '#1', '#NDDU', '#../_list/Status.txt', 'Beta', '0x01', '1', 'NEDU', 'NULL']
            ]
        });
        const parsed = parseNmmText(text, 'test.nmm');
        expect(parsed.warnings).toEqual([]);
        expect(parsed.fields).toHaveLength(2);
        expect(parsed.fields[1].notes).toBe('');
    });

    it('rejects field regions that are not a multiple of five lines', () => {
        const text = nmm({ fields: [['Alpha', '0x00', '1', 'NEDU']] });
        expect(() => parseNmmText(text, 'test.nmm')).toThrow(/multiple of 5/);
    });

    it('rejects a file count other than 1', () => {
        const text = nmm().replace('\n1\n', '\n2\n');
        expect(() => parseNmmText(text, 'test.nmm')).toThrow(/file count/);
    });
});

describe('resolveSidecarRef', () => {
    it('resolves relative to the module path, returning null for NULL', () => {
        expect(resolveSidecarRef('battle/Class.nmm', 'NULL')).toBeNull();
        const ref = resolveSidecarRef('battle/Class.nmm', '../_list/Class.txt');
        expect(ref).toMatchObject({ kind: '_list', basename: 'Class' });
        const deep = resolveSidecarRef('battle/entry/BattleUnit.nmm', '../../_list/Class.txt');
        expect(deep.sourcePath).toBe(ref.sourcePath);
    });

    it('rejects paths escaping the source tree or in unexpected locations', () => {
        expect(() => resolveSidecarRef('battle/Class.nmm', '../../etc/passwd')).toThrow(/escapes|unexpected/);
        expect(() => resolveSidecarRef('battle/Class.nmm', '../other/File.txt')).toThrow(/unexpected/);
    });

    it('detects the separator sidecar case-insensitively', () => {
        expect(isSeparatorRef(resolveSidecarRef('battle/Class.nmm', '../_list/separator.txt'))).toBe(true);
        expect(isSeparatorRef(resolveSidecarRef('battle/Class.nmm', '../_list/Separator.TXT'))).toBe(true);
        expect(isSeparatorRef(resolveSidecarRef('battle/Class.nmm', '../_list/NoYes.txt'))).toBe(false);
        expect(isSeparatorRef(null)).toBe(false);
    });
});

describe('sidecar path mapping', () => {
    it('maps _list and _name to options/, _record to entries/', () => {
        expect(mappedOptionsPath(resolveSidecarRef('battle/A.nmm', '../_list/ItemType.txt'))).toBe(
            'options/item_type.json5'
        );
        expect(mappedOptionsPath(resolveSidecarRef('battle/A.nmm', '../_name/UnitName.txt'))).toBe(
            'options/name_unit_name.json5'
        );
        expect(mappedEntriesPath(resolveSidecarRef('battle/A.nmm', '../_record/BattleUnitHeader.txt'))).toBe(
            'entries/battle_unit_header.json5'
        );
    });

    it('rejects mismatched sidecar kinds', () => {
        expect(() => mappedOptionsPath(resolveSidecarRef('battle/A.nmm', '../_record/Class.txt'))).toThrow(
            /_list or _name/
        );
        expect(() => mappedEntriesPath(resolveSidecarRef('battle/A.nmm', '../_list/Class.txt'))).toThrow(/_record/);
    });
});

describe('convertNmmText', () => {
    it('maps every type code per the spec table', () => {
        const { module } = convertNmmText(
            nmm({
                fields: [
                    ['Edit Dec', '0x00', '2', 'NEDU', 'NULL'],
                    ['Edit Signed', '0x02', '2', 'NEDS', 'NULL'],
                    ['Edit Hex', '0x04', '3', 'NEHU', 'NULL'],
                    ['Drop Dec', '0x07', '1', 'NDDU', '../_list/NoYes.txt'],
                    ['Drop Hex', '0x08', '1', 'NDHU', '../_list/NoYes.txt'],
                    ['Raw', '0x09', '4', 'HEXA', 'NULL'],
                    ['Words', '0x0D', '16', 'TEXT', 'NULL']
                ]
            }),
            'battle/Test.nmm'
        );

        const byId = Object.fromEntries(module.fields.map((f) => [f.id, f]));
        expect(byId.edit_dec).toMatchObject({ type: 'uint' });
        expect(byId.edit_dec.display).toBeUndefined();
        expect(byId.edit_signed).toMatchObject({ type: 'int' });
        expect(byId.edit_hex).toMatchObject({ type: 'uint', display: 'hex', size: 3 });
        expect(byId.drop_dec).toMatchObject({ type: 'dropdown', options_file: 'options/no_yes.json5' });
        expect(byId.drop_dec.display).toBeUndefined();
        expect(byId.drop_hex).toMatchObject({ type: 'dropdown', display: 'hex' });
        expect(byId.raw).toMatchObject({ type: 'bytes' });
        expect(byId.raw.display).toBeUndefined();
        expect(byId.words).toMatchObject({ type: 'text', size: 16 });
    });

    it('converts separator-file fields to sections, keeping the label', () => {
        const { module } = convertNmmText(
            nmm({
                fields: [
                    ['____General Flags____________', '0', '1', 'NDHU', '../_list/separator.txt'],
                    ['Alpha', '0x00', '1', 'NEDU', 'NULL']
                ]
            }),
            'battle/Test.nmm'
        );
        expect(module.fields[0]).toEqual({
            id: 'general_flags',
            label: '____General Flags____________',
            type: 'section'
        });
    });

    it('uses fallback IDs for labels that normalize to empty or non-letter starts', () => {
        const { module } = convertNmmText(
            nmm({
                fields: [
                    ['? 23 Flag ?', '0x23', '1', 'NEHU', 'NULL'],
                    ['??', '0x05', '1', 'NEDU', 'NULL'],
                    ['????', '0', '1', 'NDHU', '../_list/separator.txt'],
                    ['Named Section', '0', '1', 'NDHU', '../_list/separator.txt'],
                    ['!!', '0', '1', 'NDHU', '../_list/separator.txt']
                ]
            }),
            'battle/Test.nmm'
        );
        expect(module.fields[0].id).toBe('field_0x23');
        expect(module.fields[1].id).toBe('field_0x05');
        expect(module.fields[2].id).toBe('section_1');
        expect(module.fields[3].id).toBe('named_section');
        expect(module.fields[4].id).toBe('section_3');
    });

    it('deduplicates IDs in field order, including fallback collisions', () => {
        const { module } = convertNmmText(
            nmm({
                fields: [
                    ['Power', '0x00', '1', 'NEDU', 'NULL'],
                    ['Power', '0x01', '1', 'NEDU', 'NULL'],
                    ['Power', '0x02', '1', 'NEDU', 'NULL'],
                    ['??', '0x10', '1', 'NEDU', 'NULL'],
                    ['? ?', '0x10', '1', 'NEDU', 'NULL']
                ]
            }),
            'battle/Test.nmm'
        );
        expect(module.fields.map((f) => f.id)).toEqual(['power', 'power_2', 'power_3', 'field_0x10', 'field_0x10_2']);
    });

    it('builds module identity from the source path and title', () => {
        const { module } = convertNmmText(
            nmm({ prose: ['#A test module.'], labels: '../../_record/Class.txt' }),
            'battle/entry/BattleUnitHeader.nmm'
        );
        expect(module.id).toBe('battle_entry_battle_unit_header');
        expect(module.label).toBe('Test');
        expect(module.notes).toBe('A test module.');
        expect(module.source).toEqual({
            format: 'nightmare',
            path: 'battle/entry/BattleUnitHeader.nmm',
            title: 'Tactics Ogre: Reborn ==> battle_data_release / pack ==> Test'
        });
        expect(module.entry).toEqual({ count: 16, size: 32, labels_file: 'entries/class.json5' });
    });

    it('converts battle/entry/BattleUnit.nmm with the spec count_from block', () => {
        const { module } = convertNmmText(
            nmm({
                title: 'Tactics Ogre: Reborn ==> battle / entry / entry_unit_#### / pack ==> Battle Unit',
                baseOffset: '0x00000030',
                count: '23',
                size: '0x000000C4'
            }),
            'battle/entry/BattleUnit.nmm'
        );
        expect(module.entry).toEqual({
            count_from: { base_offset: 0x20, offset: 0x04, size: 4, type: 'uint' },
            size: 0xc4,
            labels_file: null
        });
    });

    it('rejects a dropdown without an options file and a non-dropdown with one', () => {
        expect(() =>
            convertNmmText(nmm({ fields: [['Bad Drop', '0x00', '1', 'NDDU', 'NULL']] }), 'battle/Test.nmm')
        ).toThrow(/no options file/);
        expect(() =>
            convertNmmText(
                nmm({ fields: [['Bad Edit', '0x00', '1', 'NEDU', '../_list/NoYes.txt']] }),
                'battle/Test.nmm'
            )
        ).toThrow(/non-dropdown/);
    });
});

describe('parseListText', () => {
    it('parses count then value-label pairs', () => {
        const { declaredCount, items, warnings } = parseListText('2\n0x00 No\n0x01 Yes, Aquatic\n', 'NoYes.txt');
        expect(declaredCount).toBe(2);
        expect(items).toEqual([
            { value: 0, width: 2, label: 'No' },
            { value: 1, width: 2, label: 'Yes, Aquatic' }
        ]);
        expect(warnings).toEqual([]);
    });

    it('warns when the declared count mismatches', () => {
        const { warnings } = parseListText('5\n0x00 No\n', 'X.txt');
        expect(warnings).toEqual(['X.txt: declared 5 items, parsed 1']);
    });

    it('converts value-only lines to label unknown', () => {
        const { items } = parseListText('2\n0x00\n12 Twelve\n', 'X.txt');
        expect(items[0]).toEqual({ value: 0, width: 2, label: 'unknown' });
        expect(items[1]).toEqual({ value: 12, width: 0, label: 'Twelve' });
    });

    it('rejects lines that do not start with a value', () => {
        expect(() => parseListText('1\nNo Value Here\n', 'X.txt')).toThrow(/expected "value label"/);
    });
});

describe('parseRecordText', () => {
    it('assigns positional indexes and keeps placeholder labels', () => {
        const items = parseRecordText('<<Nothing>>\nWarrior\nArcher\n');
        expect(items).toEqual([
            { value: 0, width: 0, label: '<<Nothing>>' },
            { value: 1, width: 0, label: 'Warrior' },
            { value: 2, width: 0, label: 'Archer' }
        ]);
    });
});

describe('serialization', () => {
    it('formats hex lowercase with minimum padding', () => {
        expect(formatHex(0x393f20, 8)).toBe('0x00393f20');
        expect(formatHex(4, 2)).toBe('0x04');
        expect(formatHex(0x123, 2)).toBe('0x123');
    });

    it('serializes a module exactly in the spec example style', () => {
        const { module } = convertNmmText(
            nmm({
                prose: ["#Weapon and armor stats, including units' gear."],
                labels: '../_record/Armament.txt',
                fields: [
                    ['Item Type', '0x02', '1', 'NDHU', '../_list/ItemType.txt'],
                    ['____Flags____', '0', '1', 'NDHU', '../_list/separator.txt'],
                    ['Weight', '#Heavier is slower.', '0x08', '2', 'NEDS', 'NULL']
                ]
            }),
            'battle/Test.nmm'
        );

        expect(serializeModule(module)).toBe(`{
    schema_version: 1,
    id: 'battle_test',
    label: 'Test',
    notes: 'Weapon and armor stats, including units\\' gear.',

    source: {
        format: 'nightmare',
        path: 'battle/Test.nmm',
        title: 'Tactics Ogre: Reborn ==> battle_data_release / pack ==> Test'
    },

    files: ['battle/battle_data_release.dat'],

    base_offset: 0x00000100,
    endian: 'little',

    entry: {
        count: 16,
        size: 32,
        labels_file: 'entries/armament.json5'
    },

    fields: [
        {
            id: 'item_type',
            label: 'Item Type',
            offset: 0x02,
            size: 1,
            type: 'dropdown',
            display: 'hex',
            options_file: 'options/item_type.json5'
        },
        {
            id: 'flags',
            label: '____Flags____',
            type: 'section'
        },
        {
            id: 'weight',
            label: 'Weight',
            notes: 'Heavier is slower.',
            offset: 0x08,
            size: 2,
            type: 'int'
        }
    ]
}
`);
    });

    it('serializes count_from entries', () => {
        const { module } = convertNmmText(
            nmm({
                title: 'X ==> battle / entry / entry_unit_#### / pack ==> Battle Unit',
                size: '0x000000C4'
            }),
            'battle/entry/BattleUnit.nmm'
        );
        const text = serializeModule(module);
        expect(text).toContain(`    entry: {
        count_from: {
            base_offset: 0x20,
            offset: 0x04,
            size: 4,
            type: 'uint'
        },
        size: 196,
        labels_file: null
    },`);
    });

    it('serializes sidecar items as a top-level array without a trailing comma', () => {
        const items = [
            { value: 0, width: 2, label: 'None' },
            { value: 1, width: 2, label: "Knight's Blade" }
        ];
        expect(serializeItems(items, 2)).toBe(`[
    { value: 0x00, label: 'None' },
    { value: 0x01, label: 'Knight\\'s Blade' }
]
`);
    });
});

describe('convertTree against the real reference set', () => {
    const tree = convertTree(referenceDir);

    it('converts the full inventory: 24 modules, 84 option files, 18 entry files', () => {
        expect(tree.summary).toEqual({ modules: 24, options: 84, entries: 18 });
        expect(tree.outputs.size).toBe(24 + 84 + 18);
    });

    it('produces battle_armament with the spec header values', () => {
        const text = tree.outputs.get('battle_armament.json5');
        expect(text).toContain("id: 'battle_armament'");
        expect(text).toContain('base_offset: 0x00393f20');
        expect(text).toContain('count: 761');
        expect(text).toContain('size: 152');
        expect(text).toContain("labels_file: 'entries/armament.json5'");
        expect(text).toContain("files: ['battle/battle_data_release.dat']");
    });

    it('produces the BattleUnit pair per the spec dynamic count section', () => {
        const unit = tree.outputs.get('battle_entry_battle_unit.json5');
        expect(unit).toContain("files: ['battle/entry/entry_unit_*.dat']");
        expect(unit).toContain('count_from');
        expect(unit).toContain('base_offset: 0x00000030');

        const header = tree.outputs.get('battle_entry_battle_unit_header.json5');
        expect(header).toContain('count: 1');
        expect(header).toContain('size: 16');
        expect(header).toContain('base_offset: 0x00000020');
        expect(header).toContain("type: 'text'");
    });

    it('maps menu modules to menu_data.dat', () => {
        expect(tree.outputs.get('menu_shop.json5')).toContain("files: ['menu/menu_data.dat']");
        expect(tree.outputs.get('menu_classmark.json5')).toContain("files: ['menu/menu_data.dat']");
    });

    it('keeps _list and _name outputs separate via the name_ prefix', () => {
        expect(tree.outputs.has('options/class.json5')).toBe(true);
        expect(tree.outputs.has('options/name_class.json5')).toBe(true);
        expect(tree.outputs.has('options/name_unit_name.json5')).toBe(true);
        expect(tree.outputs.has('options/unit_name.json5')).toBe(true);
    });

    it('never emits an options file for the separator', () => {
        expect(tree.outputs.has('options/separator.json5')).toBe(false);
    });

    it('converts above-label prose like the Arc/Throw comments into field notes', () => {
        const armament = tree.outputs.get('battle_armament.json5');
        expect(armament).toContain('Thrown stuff');

        const hitRate = tree.outputs.get('battle_hit_rate.json5');
        expect(hitRate).toContain("notes: 'Status 2'");
    });

    it('still warns about scratch comments that are not adjacent to a field label', () => {
        const armamentWarnings = tree.warnings.filter((w) => w.startsWith('battle/Armament.nmm'));
        expect(armamentWarnings.some((w) => w.includes('Uncomment these'))).toBe(true);
        expect(armamentWarnings.some((w) => w.includes('Thrown stuff'))).toBe(false);
    });

    it('is deterministic across runs', () => {
        const again = convertTree(referenceDir);
        expect([...again.outputs.keys()]).toEqual([...tree.outputs.keys()]);
        for (const [rel, content] of tree.outputs) {
            expect(again.outputs.get(rel)).toBe(content);
        }
    });
});
