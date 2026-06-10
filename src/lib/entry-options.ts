import type { LabeledValue } from '$lib/tauri';

export interface EntryOption {
    index: number;
    label: string;
}

/**
 * Builds the selectable entry list for a module: one option per index up to
 * the resolved count, labeled by value lookup with the first matching label.
 * Indexes without a label (or with an empty one) fall back to the numeric
 * index; label values at or above the count are never used.
 */
export function buildEntryOptions(entryCount: number, entryLabels: LabeledValue[]): EntryOption[] {
    const byValue = new Map<number, string>();
    for (const item of entryLabels) {
        if (item.value < entryCount && item.label !== '' && !byValue.has(item.value)) {
            byValue.set(item.value, item.label);
        }
    }
    return Array.from({ length: entryCount }, (_, index) => ({
        index,
        label: byValue.get(index) ?? String(index)
    }));
}
