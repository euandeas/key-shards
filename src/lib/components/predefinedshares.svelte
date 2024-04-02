<script lang="ts">
	import { Input } from '../components/input';
	import { Button } from '../components/button';
	import { invoke } from '@tauri-apps/api/tauri';
	import * as Select from '../components/select';

	export let selectedNumShares: string;
	export let share1Value: string;
	export let share2Value: string;
	export let isbip39: boolean;
	export let checkedaead: boolean;
	export let checkedpadding: boolean;
	export let secret: string;

	function handleNumSharesChange(value: string) {
		selectedNumShares = value;
		if (value === '1') {
			share2Value = '';
		}
	}

	async function pregenerateClicked(share: number) {
		if (share === 1) {
			share1Value = await invoke('generate_predefined', {
				secret: secret,
				othershare: share2Value,
				aead: checkedaead,
				isbip: isbip39,
				ispad: checkedpadding
			});
		} else if (share === 2) {
			share2Value = await invoke('generate_predefined', {
				secret: secret,
				othershare: share1Value,
				aead: checkedaead,
				isbip: isbip39,
				ispad: checkedpadding
			});
		}
	}
</script>

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
				<Input
					id="share1"
					disabled
					class="select-text pr-20 disabled:cursor-text disabled:opacity-100"
					bind:value={share1Value}
				/>
				<Button
					class="absolute inset-y-0 right-0 rounded-l-none"
					on:click={() => pregenerateClicked(1)}>Generate</Button
				>
			</div>
		{/if}
		{#if selectedNumShares === '2'}
			<div class="relative">
				<Input
					id="share2"
					disabled
					class="select-text pr-20 disabled:cursor-text disabled:opacity-100"
					bind:value={share2Value}
				/>
				<Button
					class="absolute inset-y-0 right-0 rounded-l-none"
					on:click={() => pregenerateClicked(2)}>Generate</Button
				>
			</div>
		{/if}
	</div>
</div>
