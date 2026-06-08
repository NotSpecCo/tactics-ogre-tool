<script lang="ts">
    import { decryptFile, encryptFile } from '$lib/tauri';
    import { open, save } from '@tauri-apps/plugin-dialog';

    let loading = $state(false);
    let message = $state<{ text: string; type: 'success' | 'error' } | null>(null);

    async function handleDecrypt() {
        const inputPath = await open({
            title: 'Select encrypted .dat file',
            filters: [{ name: 'DAT Files', extensions: ['dat'] }]
        });
        if (!inputPath) return;

        const outputPath = await save({
            title: 'Save decrypted ZIP as',
            defaultPath: inputPath.replace(/\.dat$/i, '.zip'),
            filters: [{ name: 'ZIP Files', extensions: ['zip'] }]
        });
        if (!outputPath) return;

        loading = true;
        message = null;
        try {
            await decryptFile(inputPath, outputPath);
            message = { text: `Decrypted to ${outputPath}`, type: 'success' };
        } catch (e) {
            message = { text: (e as Error).message ?? String(e), type: 'error' };
        } finally {
            loading = false;
        }
    }

    async function handleEncrypt() {
        const inputPath = await open({
            title: 'Select ZIP file to encrypt',
            filters: [{ name: 'ZIP Files', extensions: ['zip'] }]
        });
        if (!inputPath) return;

        const outputPath = await save({
            title: 'Save encrypted .dat as',
            defaultPath: inputPath.replace(/\.zip$/i, '.dat'),
            filters: [{ name: 'DAT Files', extensions: ['dat'] }]
        });
        if (!outputPath) return;

        loading = true;
        message = null;
        try {
            await encryptFile(inputPath, outputPath);
            message = { text: `Encrypted to ${outputPath}`, type: 'success' };
        } catch (e) {
            message = { text: (e as Error).message ?? String(e), type: 'error' };
        } finally {
            loading = false;
        }
    }
</script>

<div class="flex flex-1 items-center justify-center">
    <div class="flex w-full max-w-md flex-col gap-6 p-8">
        <div>
            <h2 class="text-lg font-semibold">Decrypt / Encrypt</h2>
            <p class="mt-1">Convert between encrypted .dat files and decrypted ZIP archives.</p>
        </div>

        {#if message}
            <div
                class="rounded px-3 py-2
					{message.type === 'success' ? 'bg-emerald-950/60 text-emerald-300' : 'bg-red-950/60 text-red-300'}"
            >
                {message.text}
            </div>
        {/if}

        <div class="flex flex-col gap-3">
            <button
                class="rounded bg-indigo-600 px-4 py-2.5 font-medium text-white hover:bg-indigo-700 disabled:opacity-50"
                disabled={loading}
                onclick={handleDecrypt}
            >
                Decrypt .dat → ZIP
            </button>
            <button
                class="rounded bg-slate-700 px-4 py-2.5 font-medium text-slate-200 hover:bg-slate-600 disabled:opacity-50"
                disabled={loading}
                onclick={handleEncrypt}
            >
                Encrypt ZIP → .dat
            </button>
        </div>

        <p class="">
            Use this to work with raw game data outside the editor. Decrypt extracts the BogoCrypt layer, giving you the
            ZIP archive. Encrypt applies BogoCrypt to a ZIP, producing a game-ready .dat.
        </p>
    </div>
</div>
