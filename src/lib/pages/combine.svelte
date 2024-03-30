<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { Button } from '../components/button';
	import { Input } from '../components/input';
	import Exit from '../icons/exit.svelte';
	import Secret from './secret.svelte';
	import { toast } from 'svelte-sonner';

	let shares: string[] = [''];
	let secret = '';

	function addShare() {
		shares = [...shares, ''];
	}

	function removeShare(index: number) {
		if (shares.length > 1) {
			shares = shares.filter((_, i) => i !== index);
		}
	}

	async function scanqr(i: number) {
		await invoke('scanqr')
			.then((res) => {
				shares[i] = res as string;
			})
			.catch((err) => {
				toast('Error', {
					description: err
				});
			});
	}

	async function upload(i: number) {
		await invoke('uploadfile')
			.then((res) => {
				shares[i] = res as string;
			})
			.catch((err) => {
				toast('Error', {
					description: err
				});
			});
	}

	function debounce<F extends (...args: any[]) => void>(fn: F, delay: number) {
		let timeoutId: ReturnType<typeof setTimeout> | undefined;
		return (...args: Parameters<F>): void => {
			clearTimeout(timeoutId);
			timeoutId = setTimeout(() => {
				fn.apply(null, args);
			}, delay);
		};
	}

	$: {
		if (shares.length > 0) {
			debouncedValidateInput();
		}
	}

	const debouncedValidateInput = debounce(validateShares, 500);

	async function validateShares() {
		let sharestrimmed = shares.map((str) => str.trim());
		let properties: [boolean, boolean, boolean, boolean][] = await invoke('check_shares', {
			list: sharestrimmed
		});
		for (let i = 0; i < sharestrimmed.length; i++) {
			let elm = document.getElementById(`shareinfo${i}`);
			if (sharestrimmed[i] !== '' && elm) {
				let [bip, short, valid, ident] = properties[i] as [boolean, boolean, boolean, boolean];
				if (bip && short) {
					elm.textContent = 'Short Share Detected';
				} else if (ident) {
					elm.textContent = 'Identical Share Detected';
				} else if (bip) {
					elm.textContent = 'BIP-39 Mnemonic Detected';
				} else if (short || valid) {
					elm.textContent = 'Likely Invalid Share Detected';
				} else {
					elm.textContent = '';
				}
			} else if (elm) {
				elm.textContent = '';
			}
		}
	}

	async function combine() {
		let sharestrimmed = shares.map((str) => str.trim());
		if (sharestrimmed.some((str) => str === '')) {
			toast('Error', {
				description: 'Cant have empty shares'
			});
			return;
		}

		await invoke('build_secret', { list: sharestrimmed })
			.then((res) => {
				if (!(res as string).trim().replace(/^\0+/, '').replace(/\0+$/, '')) {
					toast('Error', {
						description: 'Invalid Secret Generated'
					});
					return;
				}

				secret = res as string;
				showNewPage = !showNewPage;
			})
			.catch((err) => {
				toast('Error', {
					description: err
				});
			});
	}

	let showNewPage = false;
</script>

<div class="relative h-full overflow-y-hidden">
	<div class="relative h-full">
		<div class="h-full overflow-y-auto p-8">
			{#each shares as share, i}
				<div class="mb-6">
					<Input class="" bind:value={share} />
					<p class="mb-2 text-xs text-yellow-500 text-opacity-80" id="shareinfo{i}" />
					<div class="flex w-full">
						<Button
							id="uploadbutton{i}"
							class="w-full rounded-e-none px-4 py-2"
							on:click={() => upload(i)}>Upload</Button
						>
						<Button
							id="scanbutton{i}"
							class="mr-2 w-full rounded-s-none px-4 py-2"
							on:click={() => scanqr(i)}>Scan</Button
						>
						<Button class="w-18" on:click={() => removeShare(i)}>-</Button>
					</div>
				</div>
			{/each}
			<Button class="w-full" on:click={addShare}>+</Button>
		</div>
		<div
			class="absolute bottom-0 w-full p-8"
			style="background-image: linear-gradient(
			to top,
			hsl(var(--background) / var(--tw-bg-opacity)) 85%,
			hsl(var(--background) / calc(var(--tw-bg-opacity) * 0))
		  );"
		>
			<Button class="w-full" on:click={combine}>Combine</Button>
		</div>
	</div>
	<div
		class={`z-1 absolute overflow-x-hidden transition-all duration-1000 ease-in-out ${showNewPage ? 'translate-y-0' : 'translate-y-full'} top-0 h-full w-full p-8`}
		style="
    background-color: hsl(var(--background) / var(--tw-bg-opacity));"
	>
		<button
			class="absolute right-0 top-0 m-6 text-gray-500 hover:text-gray-700 focus:outline-none"
			on:click={() => (showNewPage = !showNewPage)}
		>
			<Exit />
		</button>
		<Secret {secret} />
	</div>
</div>
