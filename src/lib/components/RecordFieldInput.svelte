<script lang="ts">
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

    const isHex = $derived(field.field_type === 'Hex');
    const isUnsigned = $derived(field.value.type === 'Uint');
    const numericValue = $derived(field.value.type === 'Uint' || field.value.type === 'Int' ? field.value.value : 0);
    const hexValue = $derived(field.value.type === 'Hex' ? bytesToHex(field.value.value) : '');

    const bounds = $derived.by(() => {
        switch (field.size) {
            case 1:
                return isUnsigned ? { min: 0, max: 255 } : { min: -128, max: 127 };
            case 2:
                return isUnsigned ? { min: 0, max: 65535 } : { min: -32768, max: 32767 };
            case 4:
                return isUnsigned ? { min: 0, max: 4294967295 } : { min: -2147483648, max: 2147483647 };
            default:
                return { min: 0, max: 255 };
        }
    });

    let displayValue = $state('');
    let hasDraftHexValue = $state(false);
    let invalid = $state(false);

    const renderedHexValue = $derived(hasDraftHexValue ? displayValue : hexValue);

    function handleHexInput(e: Event) {
        displayValue = (e.target as HTMLInputElement).value;
        hasDraftHexValue = true;
        invalid = false;
    }

    function handleHexBlur() {
        const bytes = hexToBytes(displayValue);
        if (bytes && bytes.length === field.size) {
            invalid = false;
            hasDraftHexValue = false;
            onUpdate({ type: 'Hex', value: bytes });
        } else {
            invalid = true;
        }
    }

    function handleNumberChange(e: Event) {
        const target = e.target as HTMLInputElement;
        const raw = parseInt(target.value, 10);

        if (isNaN(raw)) {
            target.value = String(numericValue);
            return;
        }

        const clamped = Math.max(bounds.min, Math.min(bounds.max, raw));
        target.value = String(clamped);

        const value: FieldValue = isUnsigned ? { type: 'Uint', value: clamped } : { type: 'Int', value: clamped };
        onUpdate(value);
    }
</script>

{#if isHex}
    <Input
        type="text"
        value={renderedHexValue}
        inputClass="font-mono {invalid ? 'border-red-500' : isDirty ? 'border-indigo-400/60' : ''}"
        oninput={handleHexInput}
        onblur={handleHexBlur}
        placeholder={Array.from({ length: field.size }, () => '00').join(' ')}
    />
{:else}
    <Input
        type="number"
        value={numericValue}
        min={bounds.min}
        max={bounds.max}
        inputClass={isDirty ? 'border-indigo-400/60' : ''}
        onchange={handleNumberChange}
    />
{/if}
