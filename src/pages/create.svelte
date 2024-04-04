<script lang="ts">
	import { toast } from 'svelte-sonner';
	import { Label } from '../lib/components/label';
	import { Textarea } from '../lib/components/textarea';
	import { Input } from '../lib/components/input';
	import { Checkbox } from '../lib/components/checkbox';
	import { Button } from '../lib/components/button';
	import AdvancedOptions from '../lib/components/advancedoptions.svelte';
	import PredefinedShares from '../lib/components/predefinedshares.svelte';
	import SlideUp from '../lib/components/slideup.svelte';
	import Shares from './shares.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import ToolTip from '../lib/components/tooltipbuilder.svelte';

	$: {
		debouncedValidateInput(secret);

		if (!checkedadv) {
			checkedpre = false;
			checkedaead = true;
			checkedpadding = false;
		}

		if (
			checkedaead === true ||
			checkedaead === false ||
			checkedpadding === true ||
			checkedpadding === false ||
			checkedpre === true ||
			checkedpre === false
		) {
			share1Value = '';
			share2Value = '';
		}
	}

	let checkedadv = false;
	let checkedpre = false;
	let checkedaead = true;
	let checkedpadding = false;
	let showNewPage = false;
	let selectedNumShares = '1';
	let share1Value = '';
	let share2Value = '';
	let secret = '';
	let isbip39 = false;
	let resultingSharesComponent: Shares;

	const debouncedValidateInput = debounce(validateMnemonic, 500);

	function debounce<F extends (...args: any[]) => void>(fn: F, delay: number) {
		let timeoutId: ReturnType<typeof setTimeout> | undefined;
		return (...args: Parameters<F>): void => {
			clearTimeout(timeoutId);
			timeoutId = setTimeout(() => {
				fn.apply(null, args);
			}, delay);
		};
	}

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

	async function generateClicked() {
		let results: string[] = [];
		let predefined: string[] = [];
		let numofpredefined: number = 0;

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

		if (threshold < 3 && checkedpre) {
			toast('Error', {
				description: 'Threshold must be greater than 2 when using predefined shares.'
			});
			return;
		}

		let functionToInvoke = '';

		if (checkedadv) {
			if (isbip39) {
				if (checkedpre && checkedaead) {
					functionToInvoke = 'build_shares_bip_aead_predefined';
				} else if (checkedpre) {
					functionToInvoke = 'build_shares_bip_predefined';
				} else if (checkedaead) {
					functionToInvoke = 'build_shares_bip_aead';
				} else {
					functionToInvoke = 'build_shares_bip';
				}
			} else {
				if (checkedpre && checkedaead) {
					functionToInvoke = 'build_shares_aead_predefined';
				} else if (checkedpre) {
					functionToInvoke = 'build_shares_base_predefined';
				} else if (checkedaead) {
					functionToInvoke = 'build_shares_aead';
				} else {
					functionToInvoke = 'build_shares_base';
				}
			}
		} else {
			if (isbip39) {
				functionToInvoke = 'build_shares_bip_aead';
			} else {
				functionToInvoke = 'build_shares_aead';
			}
		}

		predefined = [share1Value, share2Value].filter((value) => value !== '');

		if (checkedpre) {
			if (share1Value === '' && share2Value === '') {
				toast('Error', {
					description: 'Predefined shares cannot be empty.'
				});
				return;
			}

			results = await invoke(functionToInvoke, {
				secret: secret,
				preshares: predefined,
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

		resultingSharesComponent.InitShares(results, predefined, numofpredefined);
		showNewPage = !showNewPage;
	}
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
				<ToolTip text={'The minimum shares needed to rebuild the secret'}>
					<Input id="threshold" pattern="[0-9]*" class="invalid:border-red-600" />
				</ToolTip>
			</div>
			<div class="grid flex-1 gap-1.5">
				<Label for="total_shares">Total Shares</Label>
				<ToolTip text={'The total number of shares produced'}>
					<Input id="total_shares" pattern="[0-9]*" class="invalid:border-red-600" />
				</ToolTip>
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
			<AdvancedOptions bind:checkedpre bind:checkedaead bind:checkedpadding />
			{#if checkedpre}
				<PredefinedShares
					bind:selectedNumShares
					bind:share1Value
					bind:share2Value
					bind:isbip39
					bind:checkedaead
					bind:checkedpadding
					bind:secret
				/>
			{/if}
		{/if}
		<Button class="mb-8 w-full" on:click={generateClicked}>Generate</Button>
	</div>
	<SlideUp bind:showNewPage>
		<Shares bind:this={resultingSharesComponent} />
	</SlideUp>
</div>
