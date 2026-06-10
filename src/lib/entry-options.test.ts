import { describe, expect, it } from 'vitest';
import { buildEntryOptions } from './entry-options';

const label = (value: number, text: string) => ({ value, label: text, notes: null });

describe('buildEntryOptions', () => {
    it('labels indexes by value lookup', () => {
        const options = buildEntryOptions(3, [label(0, 'Alpha'), label(2, 'Gamma')]);
        expect(options).toEqual([
            { index: 0, label: 'Alpha' },
            { index: 1, label: '1' },
            { index: 2, label: 'Gamma' }
        ]);
    });

    it('returns numeric labels when there are no entry labels', () => {
        expect(buildEntryOptions(2, [])).toEqual([
            { index: 0, label: '0' },
            { index: 1, label: '1' }
        ]);
    });

    it('ignores label values at or above the count', () => {
        const options = buildEntryOptions(2, [label(2, 'Out of range'), label(9, 'Way out')]);
        expect(options).toEqual([
            { index: 0, label: '0' },
            { index: 1, label: '1' }
        ]);
    });

    it('uses the first matching label for duplicate values', () => {
        const options = buildEntryOptions(1, [label(0, 'First'), label(0, 'Second')]);
        expect(options).toEqual([{ index: 0, label: 'First' }]);
    });

    it('falls back to the numeric index for empty labels', () => {
        const options = buildEntryOptions(1, [label(0, '')]);
        expect(options).toEqual([{ index: 0, label: '0' }]);
    });

    it('returns an empty list for a zero count', () => {
        expect(buildEntryOptions(0, [label(0, 'Alpha')])).toEqual([]);
    });
});
