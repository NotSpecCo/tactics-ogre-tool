import type { DisplayFormat, FieldType } from '$lib/tauri';

/**
 * Stored integer range for a field, per the spec's stored-range table.
 * Returns null for non-integer field types and invalid sizes.
 */
export function storedRange(type: FieldType, size: number | null): { min: number; max: number } | null {
    if (size === null) return null;
    switch (type) {
        case 'uint':
        case 'dropdown':
            if (size < 1 || size > 4) return null;
            return { min: 0, max: 2 ** (8 * size) - 1 };
        case 'int': {
            if (size !== 1 && size !== 2 && size !== 4) return null;
            const half = 2 ** (8 * size - 1);
            return { min: -half, max: half - 1 };
        }
        default:
            return null;
    }
}

/** Formats a stored integer per the field's display format. */
export function formatFieldValue(value: number, display: DisplayFormat | null): string {
    return display === 'hex' ? `0x${value.toString(16)}` : String(value);
}

/**
 * Parses user input as an integer. Decimal is always accepted; `0x`-prefixed
 * hex when `hex` is set (uint/dropdown accept hex regardless of display);
 * a leading minus when `negative` is set (int only). Returns null for
 * anything else — fractions, exponents, stray characters, empty input.
 */
export function parseFieldInput(raw: string, options: { hex?: boolean; negative?: boolean }): number | null {
    const trimmed = raw.trim();
    if (options.hex && /^0x[0-9a-f]+$/i.test(trimmed)) {
        return parseInt(trimmed, 16);
    }
    const decimal = options.negative ? /^-?[0-9]+$/ : /^[0-9]+$/;
    if (decimal.test(trimmed)) {
        return parseInt(trimmed, 10);
    }
    return null;
}

/** True when every character is printable ASCII (0x20 to 0x7e). */
export function isPrintableAscii(text: string): boolean {
    return /^[\x20-\x7e]*$/.test(text);
}

/**
 * Display label for a dropdown option: the label with the value in the
 * field's display format, falling back to the bare value for empty labels.
 */
export function optionLabel(label: string, value: number, display: DisplayFormat | null): string {
    const formatted = formatFieldValue(value, display);
    return label === '' ? formatted : `${label} (${formatted})`;
}
