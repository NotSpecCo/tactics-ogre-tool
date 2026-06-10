import { describe, expect, it } from 'vitest';
import { formatFieldValue, isPrintableAscii, optionLabel, parseFieldInput, storedRange } from './field-format';

describe('storedRange', () => {
    it('matches the spec table for uint and dropdown sizes', () => {
        for (const type of ['uint', 'dropdown'] as const) {
            expect(storedRange(type, 1)).toEqual({ min: 0, max: 255 });
            expect(storedRange(type, 2)).toEqual({ min: 0, max: 65535 });
            expect(storedRange(type, 3)).toEqual({ min: 0, max: 16777215 });
            expect(storedRange(type, 4)).toEqual({ min: 0, max: 4294967295 });
        }
    });

    it('matches the spec table for int sizes', () => {
        expect(storedRange('int', 1)).toEqual({ min: -128, max: 127 });
        expect(storedRange('int', 2)).toEqual({ min: -32768, max: 32767 });
        expect(storedRange('int', 4)).toEqual({ min: -2147483648, max: 2147483647 });
    });

    it('returns null for invalid sizes and non-integer types', () => {
        expect(storedRange('int', 3)).toBeNull();
        expect(storedRange('uint', 5)).toBeNull();
        expect(storedRange('uint', 0)).toBeNull();
        expect(storedRange('bytes', 4)).toBeNull();
        expect(storedRange('text', 4)).toBeNull();
        expect(storedRange('section', null)).toBeNull();
        expect(storedRange('uint', null)).toBeNull();
    });
});

describe('formatFieldValue', () => {
    it('formats hex display with a 0x prefix', () => {
        expect(formatFieldValue(31, 'hex')).toBe('0x1f');
        expect(formatFieldValue(0, 'hex')).toBe('0x0');
    });

    it('formats decimal and missing display as plain decimal', () => {
        expect(formatFieldValue(31, 'decimal')).toBe('31');
        expect(formatFieldValue(31, null)).toBe('31');
    });
});

describe('parseFieldInput', () => {
    it('parses decimal input', () => {
        expect(parseFieldInput('42', {})).toBe(42);
        expect(parseFieldInput('  42  ', {})).toBe(42);
        expect(parseFieldInput('0', {})).toBe(0);
    });

    it('parses 0x hex input when hex is allowed, regardless of case', () => {
        expect(parseFieldInput('0x1F', { hex: true })).toBe(31);
        expect(parseFieldInput('0XaB', { hex: true })).toBe(171);
    });

    it('rejects hex input when hex is not allowed', () => {
        expect(parseFieldInput('0x1f', {})).toBeNull();
    });

    it('parses negative decimal only when negative is allowed', () => {
        expect(parseFieldInput('-5', { negative: true })).toBe(-5);
        expect(parseFieldInput('-5', {})).toBeNull();
    });

    it('rejects fractions, exponents, garbage, and empty input', () => {
        expect(parseFieldInput('1.5', { hex: true })).toBeNull();
        expect(parseFieldInput('1e3', { hex: true })).toBeNull();
        expect(parseFieldInput('0x', { hex: true })).toBeNull();
        expect(parseFieldInput('12abc', { hex: true })).toBeNull();
        expect(parseFieldInput('', { hex: true })).toBeNull();
        expect(parseFieldInput('Infinity', { hex: true, negative: true })).toBeNull();
    });
});

describe('isPrintableAscii', () => {
    it('accepts printable ASCII including the boundary characters', () => {
        expect(isPrintableAscii(' ')).toBe(true);
        expect(isPrintableAscii('~')).toBe(true);
        expect(isPrintableAscii('Hello, World! 123')).toBe(true);
        expect(isPrintableAscii('')).toBe(true);
    });

    it('rejects control characters, non-ASCII, and replacement characters', () => {
        expect(isPrintableAscii('\x1f')).toBe(false);
        expect(isPrintableAscii('\x7f')).toBe(false);
        expect(isPrintableAscii('héllo')).toBe(false);
        expect(isPrintableAscii('a�b')).toBe(false);
        expect(isPrintableAscii('a\x00b')).toBe(false);
    });
});

describe('optionLabel', () => {
    it('appends the formatted value to the label', () => {
        expect(optionLabel('Sword', 1, 'decimal')).toBe('Sword (1)');
        expect(optionLabel('Sword', 31, 'hex')).toBe('Sword (0x1f)');
    });

    it('falls back to the bare formatted value for empty labels', () => {
        expect(optionLabel('', 31, 'hex')).toBe('0x1f');
        expect(optionLabel('', 31, null)).toBe('31');
    });
});
