<script lang="ts">
	import { Label } from './components/ui/label';
	import { Textarea } from './components/ui/textarea';
	import { Input } from './components/ui/input';
	import { Checkbox } from './components/ui/checkbox';
	import { Button } from './components/ui/button';
	import * as Select from './components/ui/select';
	import { invoke } from '@tauri-apps/api/tauri';

	let checkedadv = false;
	let checkedpre = false;
	let checkedaead = false;

	$: {
		// Uncheck "Predefined Shares" when "Advanced" is unchecked
		if (!checkedadv) {
			checkedpre = false;
		}

		if (!checkedpre) {
			share1Value = '';
			share2Value = '';
		}
	}

	let selectedNumShares = '1';
	let share1Value = '';
	let share2Value = '';

	function handleNumSharesChange(value: string) {
		selectedNumShares = value;

		if (value === '1') {
			share2Value = '';
		}
	}
</script>

<div>
	<div class="mb-8 grid w-full gap-1.5">
		<Label for="secret">Secret</Label>
		<Textarea class="resize-none" id="secret" />
	</div>
	<div class="mb-8 flex">
		<div class="mr-6 grid flex-1 gap-1.5">
			<Label for="threshold">Threshold</Label>
			<Input id="threshold" />
		</div>
		<div class="grid flex-1 gap-1.5">
			<Label for="total_shares">Total Shares</Label>
			<Input id="total_shares" />
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
			<div class="flex items-center space-x-2">
				<Checkbox id="aead" bind:checked={checkedaead} />
				<Label
					for="aead"
					class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
				>
					XChaCha20-Poly1305 AEAD
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
							<Input id="share1" disabled class="pr-20" />
							<Button class="absolute inset-y-0 right-0 rounded-l-none">Generate</Button>
						</div>
					{/if}
					{#if selectedNumShares === '2'}
						<div class="relative">
							<Input id="share2" disabled class="pr-20" />
							<Button class="absolute inset-y-0 right-0 rounded-l-none">Generate</Button>
						</div>
					{/if}
				</div>
			</div>
		{/if}
	{/if}

	<Button class="mb-8 w-full">Generate</Button>
</div>
