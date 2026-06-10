import { describe, expect, it } from 'vitest';
import { buildEntryOptions } from './entry-options';

const label = (value: number, text: string, notes: string | null = null) => ({ value, label: text, notes });

describe('buildEntryOptions', () => {
    it('labels indexes by value lookup', () => {
        const options = buildEntryOptions(3, [label(0, 'Alpha'), label(2, 'Gamma')]);
        expect(options).toEqual([
            { index: 0, label: 'Alpha', notes: null },
            { index: 1, label: null, notes: null },
            { index: 2, label: 'Gamma', notes: null }
        ]);
    });

    it('returns unlabeled options when there are no entry labels', () => {
        expect(buildEntryOptions(2, [])).toEqual([
            { index: 0, label: null, notes: null },
            { index: 1, label: null, notes: null }
        ]);
    });

    it('ignores label values at or above the count', () => {
        const options = buildEntryOptions(2, [label(2, 'Out of range'), label(9, 'Way out')]);
        expect(options).toEqual([
            { index: 0, label: null, notes: null },
            { index: 1, label: null, notes: null }
        ]);
    });

    it('uses the first matching label for duplicate values', () => {
        const options = buildEntryOptions(1, [label(0, 'First'), label(0, 'Second')]);
        expect(options).toEqual([{ index: 0, label: 'First', notes: null }]);
    });

    it('falls back to a null label for empty labels', () => {
        const options = buildEntryOptions(1, [label(0, '', 'still noted')]);
        expect(options).toEqual([{ index: 0, label: null, notes: 'still noted' }]);
    });

    it('carries entry notes through', () => {
        const options = buildEntryOptions(1, [label(0, 'Alpha', 'The first one.')]);
        expect(options).toEqual([{ index: 0, label: 'Alpha', notes: 'The first one.' }]);
    });

    it('returns an empty list for a zero count', () => {
        expect(buildEntryOptions(0, [label(0, 'Alpha')])).toEqual([]);
    });
});
