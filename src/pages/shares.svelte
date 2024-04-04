<script lang="ts">
	import { Button } from '../lib/components/button';
	import { Input } from '../lib/components/input';
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import CopyIcon from '../lib/icons/copy.svelte';

	export let results: string[] = [];

	export function InitShares(sentresults: string[], predefined: string[], numofpredefined: number) {
		results = sentresults;
		resultasb64 = [...results];
		resultsasbip = [];

		numofpred = numofpredefined;
		pred = predefined;

		for (let i = 0; i < numofpredefined; i++) {
			resultasb64[i] = pred[i];
		}
	}

	onMount(() => {
		InitShares(results, [], 0);
	});

	let resultasb64: string[] = [];
	let resultsasbip: string[] = [];
	let numofpred: number = 0;
	let pred: string[];

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

	async function bytelength(i: number): Promise<[boolean, boolean, boolean]> {
		if (i < numofpred) {
			return [true, true, true];
		} else {
			let size: number = await invoke('bytelength', { base64: results[i] });
			if (size > 32) {
				return [true, false, false];
			}
			return [false, false, false];
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
	{#key results}
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
					{#await bytelength(i) then disabled}
						<Button
							id="bipbutton{i}"
							class="w-full rounded-e-none px-4 py-2"
							on:click={() => toggleBIP(i)}
							disabled={disabled[0]}
						>
							{#if disabled[0] === true && disabled[1] === true && disabled[2] === true}
								Base64
							{:else}
								BIP
							{/if}
						</Button>
						<Button
							id="filebutton{i}"
							class="border-grey w-full rounded-none border-x px-4 py-2"
							on:click={() => exportasfile(i)}
							disabled={disabled[1]}>File</Button
						>
						<Button
							id="qrbutton{i}"
							class="w-full rounded-s-none px-4 py-2"
							on:click={() => exportasqr(i)}
							disabled={disabled[2]}>QR</Button
						>
					{/await}
				</div>
			</div>
		{/each}
	{/key}
</div>
