<script lang="ts">
	import { toast } from 'svelte-sonner';
	import { Label } from '../components/ui/label';
	import { Textarea } from '../components/ui/textarea';
	import { Input } from '../components/ui/input';
	import { Checkbox } from '../components/ui/checkbox';
	import { Button } from '../components/ui/button';
	import Result from '../pages/result.svelte';
	import * as Select from '../components/ui/select';
	import { invoke } from '@tauri-apps/api/tauri';

	let checkedadv = false;
	let checkedpre = false;
	let checkedaead = false;
	let checkedpadding = false;

	$: {
		debouncedValidateInput(secret);
		// Uncheck "Predefined Shares" when "Advanced" is unchecked
		if (!checkedadv) {
			checkedpre = false;
		}

		if (checkedaead === true || checkedaead === false || checkedpadding === true || checkedpadding === false || checkedpre === true || checkedpre === false) {
			share1Value = '';
			share2Value = '';
		}
	}

	let selectedNumShares = '1';
	let share1Value = '';
	let share2Value = '';

	async function handleNumSharesChange(value: string) {
		selectedNumShares = value;

		if (value === '1') {
			share2Value = '';
		}
	}

	let secret = '';
	let isbip39 = false;

	function debounce<F extends (...args: any[]) => void>(fn: F, delay: number) {
		let timeoutId: ReturnType<typeof setTimeout> | undefined;
		return (...args: Parameters<F>): void => {
			clearTimeout(timeoutId);
			timeoutId = setTimeout(() => {
				fn.apply(null, args);
			}, delay);
		};
	}

	const debouncedValidateInput = debounce(validateMnemonic, 500);

	async function validateMnemonic(secret: string) {
		secret = secret.trim();
		let elm = document.getElementById('secretInfo');
		if (secret !== '' && elm) {
			if (await invoke('verify_mnemonic', { mnemonic: secret })) {
				elm.textContent = 'BIP-39 compression will be applied.';
				isbip39 = true;
			} else {
				elm.textContent = '';
				isbip39 = false;
			}
		} else if (elm) {
			elm.textContent = '';
			isbip39 = false;
		}

		share1Value = '';
		share2Value = '';
	}

	let results: string[] = [];
	let numofpredefined: number = 0;

	async function generateClicked() {
		const threshold = Number((document.getElementById('threshold') as HTMLInputElement).value);
		const total_shares: number = Number(
			(document.getElementById('total_shares') as HTMLInputElement).value
		);

		if (secret === '') {
			toast('Error', {
				description: 'Secret cannot be empty.'
			});
			return;
		}

		if (Number.isNaN(threshold) || Number.isNaN(total_shares)) {
			toast('Error', {
				description: 'Shares must be a number.'
			});
			return;
		}

		if (threshold === 0 || total_shares === 0) {
			toast('Error', {
				description: 'Shares must be greater than 0.'
			});
			return;
		}

		if (threshold > total_shares) {
			toast('Error', {
				description: 'Threshold must be less than or equal to total shares.'
			});
			return;
		}

		let functionToInvoke = '';

		if (checkedadv) {
			if (isbip39) {
				if (checkedpre && checkedaead){
					functionToInvoke = 'build_shares_bip_aead_predefined';
				} else if (checkedpre){
					functionToInvoke = 'build_shares_bip_predefined';
				}
				else if (checkedaead){
					functionToInvoke = 'build_shares_bip_aead';
				}
				else{
					functionToInvoke = 'build_shares_bip';
				}
			} else {
				if (checkedpre && checkedaead){
					functionToInvoke = 'build_shares_aead_predefined';
				} else if (checkedpre){
					functionToInvoke = 'build_shares_base_predefined';
				}
				else if (checkedaead){
					functionToInvoke = 'build_shares_aead';
				}
				else{
					functionToInvoke = 'build_shares_base';
				}
			}
		} else {
			if (isbip39) {
				functionToInvoke = 'build_shares_bip';
			} else {
				functionToInvoke = 'build_shares_base';
			}
		}

		if (checkedpre) {
			if (share1Value === '' && share2Value === '') {
				toast('Error', {
					description: 'Predefined shares cannot be empty.'
				});
				return;
			}

			results = await invoke(functionToInvoke, {
				secret: secret,
				preshares: [share1Value, share2Value].filter((value) => value !== ''),
				threshold: threshold,
				totalshares: total_shares,
				pad: checkedpadding
			});
		} else {
			results = await invoke(functionToInvoke, {
				secret: secret,
				threshold: threshold,
				totalshares: total_shares,
				pad: checkedpadding
			});
		}

		if (checkedpre && share1Value !== '' && share2Value !== '') {
			numofpredefined = 2;
		} else if (checkedpre && (share1Value !== '' || share2Value !== '')) {
			numofpredefined = 1;
		} else {
			numofpredefined = 0;
		}

		showNewPage = !showNewPage;
	}

	async function pregenerateClicked(share: number) {
		if (share === 1){
			share1Value = await invoke('generate_predefined', {secret: secret, othershare: share2Value, aead: checkedaead, isbip: isbip39, ispad: checkedpadding});
		} else if (share === 2){
			share2Value = await invoke('generate_predefined', {secret: secret, othershare: share1Value, aead: checkedaead, isbip: isbip39, ispad: checkedpadding});
		}
	}

	let showNewPage = false;
</script>

<div class="relative h-full overflow-y-hidden">
	<div class="z-0 h-full select-none p-8">
		<div class="mb-8 grid w-full gap-1.5">
			<Label for="secret">Secret</Label>
			<Textarea class="resize-none" id="secret" bind:value={secret} />
			<p class="text-xs text-green-500 text-opacity-80" id="secretInfo" />
		</div>
		<div class="mb-8 flex">
			<div class="mr-6 grid flex-1 gap-1.5">
				<Label for="threshold">Threshold</Label>
				<Input id="threshold" pattern="[0-9]*" class="invalid:border-red-600" />
			</div>
			<div class="grid flex-1 gap-1.5">
				<Label for="total_shares">Total Shares</Label>
				<Input id="total_shares" pattern="[0-9]*" class="invalid:border-red-600" />
			</div>
			<div class="flex flex-1 items-center justify-end space-x-2">
				<Checkbox id="advanced" bind:checked={checkedadv} />
				<Label
					for="advanced"
					class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
				>
					Advanced
				</Label>
			</div>
		</div>
		{#if checkedadv}
			<div class="mb-8 flex">
				<div class="mr-6 flex items-center space-x-2">
					<Checkbox id="predefined" bind:checked={checkedpre} />
					<Label
						for="predefined"
						class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
					>
						Predefined Shares
					</Label>
				</div>
				<div class="mr-6 flex items-center space-x-2">
					<Checkbox id="aead" bind:checked={checkedaead} />
					<Label
						for="aead"
						class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
					>
						XChaCha20-Poly1305 AEAD
					</Label>
				</div>
				<div class="flex items-center space-x-2">
					<Checkbox id="padding" bind:checked={checkedpadding} />
					<Label
						for="padding"
						class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
					>
						Padding
					</Label>
				</div>
			</div>
			{#if checkedpre}
				<div class="mb-8">
					<div class="mb-4">
						<Select.Root>
							<Select.Label class="pl-0">Number of Predefined Shares</Select.Label>
							<Select.Trigger class="w-[180px]">
								<Select.Value placeholder="1" />
							</Select.Trigger>
							<Select.Content>
								<Select.Item value="1" on:click={() => handleNumSharesChange('1')}>1</Select.Item>
								<Select.Item value="2" on:click={() => handleNumSharesChange('2')}>2</Select.Item>
							</Select.Content>
						</Select.Root>
					</div>
					<div class="grid grid-cols-2 gap-4">
						{#if selectedNumShares === '1' || selectedNumShares === '2'}
							<div class="relative">
								<Input id="share1" disabled class="pr-20" bind:value={share1Value}/>
								<Button class="absolute inset-y-0 right-0 rounded-l-none" on:click={() => pregenerateClicked(1)}>Generate</Button>
							</div>
						{/if}
						{#if selectedNumShares === '2'}
							<div class="relative">
								<Input id="share2" disabled class="pr-20" bind:value={share2Value}/>
								<Button class="absolute inset-y-0 right-0 rounded-l-none" on:click={() => pregenerateClicked(2)}>Generate</Button>
							</div>
						{/if}
					</div>
				</div>
			{/if}
		{/if}

		<Button class="mb-8 w-full" on:click={generateClicked}>Generate</Button>
	</div>
	<div
		class={`z-1 absolute overflow-x-hidden transition-all duration-1000 ease-in-out ${showNewPage ? 'translate-y-0' : 'translate-y-full'} top-0 h-full w-full p-8`}
		style="
    background-color: hsl(var(--background) / var(--tw-bg-opacity));"
	>
		<button
			class="absolute right-0 top-0 text-gray-500 hover:text-gray-700 focus:outline-none m-6"
			on:click={() => (showNewPage = !showNewPage)}
		>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				class="h-6 w-6"
				fill="none"
				viewBox="0 0 24 24"
				stroke="currentColor"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M6 18L18 6M6 6l12 12"
				/>
			</svg>
		</button>
		<Result {results} {numofpredefined} />
	</div>
</div>
