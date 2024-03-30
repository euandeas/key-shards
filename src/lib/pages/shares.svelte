<script lang="ts">
	import { Button } from '../components/ui/button';
	import { Input } from '../components/ui/input';
	import { invoke } from '@tauri-apps/api/tauri';

	export let results: string[] = [];
	export let predefined: string[] = [];
	let resultasb64: string[] = [];
	let resultsasbip: string[] = [];
	export let numofpredefined: number = 0;

	$: {
		if (results.length === 0 || results.length !== 0) {
			resultasb64 = [...results];
			resultsasbip = [];

			for (let i = 0; i < results.length; i++) {
				const bipButton = document.getElementById(`bipbutton${i}`) as HTMLButtonElement;
				const fileButton = document.getElementById(`filebutton${i}`) as HTMLButtonElement;
				const qrButton = document.getElementById(`qrbutton${i}`) as HTMLButtonElement;
				if (bipButton != null) {
					bipButton.textContent = 'BIP';
					bipButton.disabled = false;
					fileButton.disabled = false;
					qrButton.disabled = false;
				}
				bytelength(i);
			}

			for (let i = 0; i < numofpredefined; i++) {
				const bipButton = document.getElementById(`bipbutton${i}`) as HTMLButtonElement;
				const fileButton = document.getElementById(`filebutton${i}`) as HTMLButtonElement;
				const qrButton = document.getElementById(`qrbutton${i}`) as HTMLButtonElement;
				if (bipButton != null) {
					resultasb64[i] = predefined[i];
					bipButton.textContent = 'Base64';
					bipButton.disabled = true;
					fileButton.disabled = true;
					qrButton.disabled = true;
				}
			}
		}
	}

	async function toggleBIP(i: number) {
		const bipButton = document.getElementById(`bipbutton${i}`) as HTMLButtonElement;

		if (bipButton.textContent === 'BIP') {
			bipButton.textContent = 'Base64';
			let temp = resultasb64[i];
			if (resultsasbip[i] === undefined) {
				resultsasbip[i] = await invoke('calculate_bip39', { base64: temp });
			}
			resultasb64[i] = resultsasbip[i];
			resultsasbip[i] = temp;
		} else {
			bipButton.textContent = 'BIP';
			let temp = resultsasbip[i];
			resultsasbip[i] = resultasb64[i];
			resultasb64[i] = temp;
		}
	}

	async function bytelength(num: number) {
		if ((num === 0 || num === 1) && numofpredefined > 0) {
			return;
		}

		let size: number = await invoke('bytelength', { base64: results[num] });
		if (size > 32) {
			const bipButton = document.getElementById(`bipbutton${num}`) as HTMLButtonElement;
			if (bipButton != null) {
				bipButton.disabled = true;
			}
		}
	}

	async function exportasfile(i: number) {
		await invoke('exportaspem', { base64: results[i] });
	}

	async function exportasqr(i: number) {
		await invoke('exportasqr', { base64: results[i] });
	}
</script>

<div class="mt-12">
	{#each resultasb64 as result, i}
		<div class="mb-6">
			<Input
				value={result}
				disabled
				class="mb-2 select-text disabled:cursor-text disabled:opacity-100"
			/>
			<div class="flex w-full">
				<Button
					id="bipbutton{i}"
					class="w-full rounded-e-none px-4 py-2"
					on:click={() => toggleBIP(i)}>BIP</Button
				>
				<Button
					id="filebutton{i}"
					class="border-grey w-full rounded-none border-x px-4 py-2"
					on:click={() => exportasfile(i)}>File</Button
				>
				<Button
					id="qrbutton{i}"
					class="w-full rounded-s-none px-4 py-2"
					on:click={() => exportasqr(i)}>QR</Button
				>
			</div>
		</div>
	{/each}
</div>
