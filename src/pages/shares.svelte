<script lang="ts">
	import { Button } from '../lib/components/button';
	import { Input } from '../lib/components/input';
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import CopyIcon from '../lib/icons/copy.svelte';

	export let results: string[] = ['', ''];

	export function InitShares(sentresults: string[], predefined: string[], numofpredefined: number) {
		results = sentresults;
		resultasb64 = [...results];
		resultsasbip = [];

		for (let i = 0; i < results.length; i++) {
			const bipButton = document.getElementById(`bipbutton${i}`) as HTMLButtonElement;
			const fileButton = document.getElementById(`filebutton${i}`) as HTMLButtonElement;
			const qrButton = document.getElementById(`qrbutton${i}`) as HTMLButtonElement;

			if (bipButton != null) {
				if (i < numofpredefined) {
					resultasb64[i] = predefined[i];
					bipButton.textContent = 'Base64';
					bipButton.disabled = true;
					fileButton.disabled = true;
					qrButton.disabled = true;
				} else {
					bipButton.textContent = 'BIP';
					bipButton.disabled = false;
					fileButton.disabled = false;
					qrButton.disabled = false;

					bytelength(i, bipButton);
				}
			}
		}
	}

	onMount(() => {
		InitShares(results, [], 0);
	});

	let resultasb64: string[] = [];
	let resultsasbip: string[] = [];

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

	async function bytelength(num: number, button: HTMLButtonElement) {
		let size: number = await invoke('bytelength', { base64: results[num] });
		if (size > 32) {
			button.disabled = true;
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
			<div class="relative">
				<Input
					value={result}
					disabled
					class="mb-2 select-text pr-12 disabled:cursor-text disabled:opacity-100"
				/>
				<CopyIcon
					class="absolute inset-y-0 right-0 flex cursor-pointer items-center px-3 text-gray-500 hover:text-gray-700"
					text={result}
				/>
			</div>
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
