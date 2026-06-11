<script lang="ts">
    import { formatFieldValue, isPrintableAscii, parseFieldInput, storedRange } from '$lib/field-format';
    import type { FieldValue, RecordField } from '$lib/tauri';
    import Input from '$lib/ui-components/form/Input.svelte';

    interface Props {
        field: RecordField;
        isDirty: boolean;
        onUpdate: (value: FieldValue) => void;
    }

    let { field, isDirty, onUpdate }: Props = $props();

    function bytesToHex(bytes: number[]): string {
        return bytes.map((b) => b.toString(16).toUpperCase().padStart(2, '0')).join(' ');
    }

    function hexToBytes(hex: string): number[] | null {
        const clean = hex.replace(/\s+/g, '');
        if (clean.length % 2 !== 0 || !/^[0-9a-fA-F]*$/.test(clean)) return null;

        const bytes: number[] = [];
        for (let i = 0; i < clean.length; i += 2) {
            bytes.push(parseInt(clean.substring(i, i + 2), 16));
        }

        return bytes;
    }

    const size = $derived(field.size ?? 0);
    const isBytes = $derived(field.type === 'bytes');
    const isText = $derived(field.type === 'text');
    const isInt = $derived(field.type === 'int');

    /** True when the stored text decoded lossily and must not be written back. */
    const lossyText = $derived(field.value?.type === 'text' && field.value.value.includes('�'));

    // The user's in-progress input. While null, the input renders the stored
    // value; a failed commit keeps the draft on screen with error styling.
    let draft = $state<string | null>(null);
    let invalid = $state(false);

    const rendered = $derived.by(() => {
        if (draft !== null) return draft;
        switch (field.value?.type) {
            case 'uint':
                return formatFieldValue(field.value.value, field.display);
            case 'int':
                return String(field.value.value);
            case 'bytes':
                return bytesToHex(field.value.value);
            case 'text':
                return field.value.value;
            default:
                return '';
        }
    });

    const title = $derived.by(() => {
        if (isText) {
            const hint = `Printable ASCII only, up to ${size} character${size === 1 ? '' : 's'}.`;
            return lossyText ? `${hint} The current value contains non-text bytes and is display-only.` : hint;
        }
        if (isBytes) return `${size} byte${size === 1 ? '' : 's'} of hex.`;
        const range = storedRange(field.type, field.size);
        if (!range) return undefined;
        const hex = isInt ? '' : '; decimal or 0x hex';
        return `${range.min} to ${range.max}${hex}.`;
    });

    function handleInput(e: Event) {
        draft = (e.target as HTMLInputElement).value;
        invalid = false;
    }

    function commit() {
        if (draft === null) return;
        const raw = draft;

        if (isBytes) {
            const bytes = hexToBytes(raw);
            if (!bytes || bytes.length !== size) {
                invalid = true;
                return;
            }
            draft = null;
            onUpdate({ type: 'bytes', value: bytes });
        } else if (isText) {
            // Sends only what the user typed, never the lossy read-back of
            // the stored bytes.
            if (!isPrintableAscii(raw) || raw.length > size) {
                invalid = true;
                return;
            }
            draft = null;
            onUpdate({ type: 'text', value: raw });
        } else {
            const parsed = parseFieldInput(raw, { hex: !isInt, negative: isInt });
            const range = storedRange(field.type, field.size);
            if (parsed === null || !range || parsed < range.min || parsed > range.max) {
                invalid = true;
                return;
            }
            draft = null;
            onUpdate(isInt ? { type: 'int', value: parsed } : { type: 'uint', value: parsed });
        }
    }
</script>

<Input
    type="text"
    value={rendered}
    {title}
    maxlength={isText ? size : undefined}
    placeholder={isBytes ? Array.from({ length: size }, () => '00').join(' ') : undefined}
    inputClass="{isBytes || isText ? 'font-mono' : ''}
        {invalid ? 'border-red-500' : isDirty ? 'border-indigo-400/60' : ''}
        {lossyText && !invalid ? 'border-amber-500' : ''}"
    oninput={handleInput}
    onchange={commit}
/>
