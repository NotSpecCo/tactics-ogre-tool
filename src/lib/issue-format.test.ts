import { describe, expect, it } from 'vitest';
import { issueLocation } from './issue-format';

describe('issueLocation', () => {
    it('shows only the file when no ids are present', () => {
        expect(issueLocation({ file: 'battle_class.json5', module_id: null, field_id: null, message: 'm' })).toBe(
            'battle_class.json5'
        );
    });

    it('appends the module id', () => {
        expect(
            issueLocation({ file: 'battle_class.json5', module_id: 'battle_class', field_id: null, message: 'm' })
        ).toBe('battle_class.json5 · battle_class');
    });

    it('appends module and field ids', () => {
        expect(
            issueLocation({
                file: 'battle_class.json5',
                module_id: 'battle_class',
                field_id: 'unknown_0x00',
                message: 'm'
            })
        ).toBe('battle_class.json5 · battle_class · unknown_0x00');
    });

    it('skips a field id with no module id', () => {
        expect(issueLocation({ file: 'options/skill.json5', module_id: null, field_id: 'skill', message: 'm' })).toBe(
            'options/skill.json5 · skill'
        );
    });
});
