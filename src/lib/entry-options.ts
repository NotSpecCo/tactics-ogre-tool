import type { LabeledValue } from '$lib/tauri';

export interface EntryOption {
    index: number;
    /** Null when the index has no label (or an empty one); display the index. */
    label: string | null;
    notes: string | null;
}

/**
 * Builds the selectable entry list for a module: one option per index up to
 * the resolved count, labeled by value lookup with the first matching label.
 * Indexes without a label (or with an empty one) fall back to the numeric
 * index; label values at or above the count are never used.
 */
export function buildEntryOptions(entryCount: number, entryLabels: LabeledValue[]): EntryOption[] {
    const byValue = new Map<number, LabeledValue>();
    for (const item of entryLabels) {
        if (item.value < entryCount && !byValue.has(item.value)) {
            byValue.set(item.value, item);
        }
    }
    return Array.from({ length: entryCount }, (_, index) => {
        const item = byValue.get(index);
        return {
            index,
            label: item && item.label !== '' ? item.label : null,
            notes: item?.notes ?? null
        };
    });
}
