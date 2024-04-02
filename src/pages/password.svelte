<script lang="ts">
	import { toast } from 'svelte-sonner';
	import { Label } from '../lib/components/label';
	import { Input } from '../lib/components/input';
	import { Checkbox } from '../lib/components/checkbox';
	import { Button } from '../lib/components/button';
	import { Slider } from '../lib/components/slider';
	import * as Select from '../lib/components/select';
	import * as Tabs from '../lib/components/tabs';
	import { onMount } from 'svelte';

	import { invoke } from '@tauri-apps/api/tauri';

	$: formattedValue = sliderValue[0].toString();
	$: displayText =
		option === 'password' ? `${formattedValue} Characters` : `${formattedValue} Words`;

	let generatedValue: string = '';
	let option = 'password';
	let showSlide = true;

	let sliderValue: number[] = [16];
	let sliderMin = 5;
	let sliderMax = 128;

	let bipWords = 15;

	let uppercaseChecked = true;
	let lowercaseChecked = true;
	let numbersChecked = true;
	let symbolsChecked = true;
	let ambiguousChecked = true;
	let capitaliseChecked = true;
	let seperator = '-';

	function optionChanged(value: string) {
		option = value;

		let elm = document.getElementById('passInfo');

		if (value === 'password') {
			showSlide = true;
			sliderValue = [16];
			sliderMin = 5;
			sliderMax = 128;

			if (elm) {
				clearElm(elm);
			}
		} else if (value === 'passphrase') {
			showSlide = true;
			sliderValue = [5];
			sliderMin = 3;
			sliderMax = 20;

			if (elm) {
				clearElm(elm);
			}
		} else {
			if (elm) {
				clearElm(elm);
			}
			showSlide = false;
		}
	}

	function clearElm(elm: HTMLElement) {
		elm.innerText = '';
		elm.classList.remove('text-red-500');
		elm.classList.remove('text-yellow-500');
		elm.classList.remove('text-green-500');
	}

	function bipWordsChanged(value: number) {
		bipWords = value;

		let elm = document.getElementById('passInfo');

		if ((value === 3 || value === 6 || value === 9) && elm) {
			clearElm(elm);
			elm.innerText =
				'Mnemonics with less than 12 words have low entropy and may be guessed by an attacker.';
			elm.classList.add('text-yellow-500');
		} else if (elm) {
			clearElm(elm);
		}
	}

	function seperatorChanged(value: string) {
		seperator = value;
	}

	function calculatePasswordEntropy(password: string) {
		let characterRange = 0;
		let numericFlag = false;
		let lowercaseFlag = false;
		let uppercaseFlag = false;
		let specialFlag = false;

		// Iterate through each character in the password
		for (const char of password) {
			// Check if the character is a numeric and numericFlag is false
			if (/[0-9]/.test(char) && !numericFlag) {
				characterRange += 10; // Numerics range size is 10
				numericFlag = true;
			}
			// Check if the character is a lowercase Latin letter and lowercaseFlag is false
			else if (/[a-z]/.test(char) && !lowercaseFlag) {
				characterRange += 26; // Lowercase Latin letters range size is 26
				lowercaseFlag = true;
			}
			// Check if the character is an uppercase Latin letter and uppercaseFlag is false
			else if (/[A-Z]/.test(char) && !uppercaseFlag) {
				characterRange += 26; // Uppercase Latin letters range size is 26
				uppercaseFlag = true;
			}
			// Check if the character is one of the specified special symbols and specialFlag is false
			else if ('!@#$%^&*'.includes(char) && !specialFlag) {
				characterRange += 8; // Special symbols range size is 8
				specialFlag = true;
			}

			if (numericFlag && lowercaseFlag && uppercaseFlag && specialFlag) {
				break;
			}
		}

		console.log(characterRange);

		//Calculate the log2 of the character range
		const log2CharacterRange = Math.log2(characterRange);

		// Calculate the password entropy using the formula
		const passwordEntropy = password.length * log2CharacterRange;

		return passwordEntropy;
	}

	function calculatePassphraseEntropy() {
		// Calculate the log2 of the wordlist size
		const log2WordlistSize = Math.log2(1296);

		// Calculate the passphrase entropy using the formula
		const passphraseEntropy = log2WordlistSize * sliderValue[0];

		return passphraseEntropy;
	}

	function checkEntropy(password: string) {
		let elm = document.getElementById('passInfo');
		let entropy = 0;

		if (elm === null) {
			return;
		}

		if (option === 'password') {
			entropy = calculatePasswordEntropy(password);
		} else if (option === 'passphrase') {
			entropy = calculatePassphraseEntropy();
		}

		if (entropy < 35) {
			clearElm(elm);
			elm.innerText = 'Vunerable. Consider using a longer password or passphrase.';
			elm.classList.add('text-red-500');
		} else if (entropy < 60) {
			clearElm(elm);
			elm.innerText = 'Weak. Consider using a longer password or passphrase.';
			elm.classList.add('text-yellow-500');
		} else if (entropy > 100) {
			clearElm(elm);
			elm.innerText = 'Very Strong';
			elm.classList.add('text-green-500');
		} else {
			clearElm(elm);
			elm.innerText = 'Strong.';
			elm.classList.add('text-green-500');
		}
	}

	async function regenerate() {
		if (option === 'password') {
			generatedValue = await invoke('generate_password', {
				length: sliderValue[0],
				uppercase: uppercaseChecked,
				lowercase: lowercaseChecked,
				numbers: numbersChecked,
				symbols: symbolsChecked,
				ambiguous: ambiguousChecked
			});

			checkEntropy(generatedValue);
		} else if (option === 'passphrase') {
			generatedValue = await invoke('generate_passphrase', {
				length: sliderValue[0],
				seperator: seperator,
				capitalise: capitaliseChecked
			});

			checkEntropy(generatedValue);
		} else {
			generatedValue = await invoke('generate_bip', {
				words: bipWords
			});
		}
	}
</script>

<div class="relative h-full p-8">
	<Tabs.Root value="password" class="mb-4 w-full">
		<Tabs.List>
			<Tabs.Trigger value="password" on:click={() => optionChanged('password')}
				>Password</Tabs.Trigger
			>
			<Tabs.Trigger value="passphrase" on:click={() => optionChanged('passphrase')}
				>Passphrase</Tabs.Trigger
			>
			<Tabs.Trigger value="bip" on:click={() => optionChanged('bip')}>BIP Mnemonic</Tabs.Trigger>
		</Tabs.List>
	</Tabs.Root>
	<div class="mb-8">
		<h4
			class="mb-2 min-h-16 scroll-m-20 break-all rounded-md border border-input p-4 text-center text-xl font-semibold tracking-tight"
		>
			{generatedValue}
		</h4>
		<p class="text-xs text-opacity-80" id="passInfo" />
	</div>
	{#if showSlide}
		<div class="mb-8 grid w-full gap-4">
			<Label for="slide">{displayText}</Label>
			<Slider id="slide" bind:value={sliderValue} min={sliderMin} max={sliderMax} step={1} />
		</div>
	{/if}
	<div class="mb-8 flex">
		{#if option === 'password'}
			<div class="mr-6 flex items-center space-x-2">
				<Checkbox id="uppercase" bind:checked={uppercaseChecked} />
				<Label
					for="uppercase"
					class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
				>
					A-Z
				</Label>
			</div>
			<div class="mr-6 flex items-center space-x-2">
				<Checkbox id="lowercase" bind:checked={lowercaseChecked} />
				<Label
					for="lowercase"
					class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
				>
					a-z
				</Label>
			</div>
			<div class="mr-6 flex items-center space-x-2">
				<Checkbox id="numbers" bind:checked={numbersChecked} />
				<Label
					for="numbers"
					class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
				>
					0-9
				</Label>
			</div>
			<div class="mr-6 flex items-center space-x-2">
				<Checkbox id="symbols" bind:checked={symbolsChecked} />
				<Label
					for="symbols"
					class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
				>
					!@#$%^&*
				</Label>
			</div>
			<div class="flex items-center space-x-2">
				<Checkbox id="ambiguous" bind:checked={ambiguousChecked} />
				<Label
					for="ambiguous"
					class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
				>
					Avoid Ambiguous Characters
				</Label>
			</div>
		{:else if option === 'passphrase'}
			<div class="mr-6 flex items-center space-x-2">
				<Select.Root>
					<Select.Label class="pl-0">Seperator</Select.Label>
					<Select.Trigger class="w-[180px]">
						<Select.Value placeholder="Hyphen" />
					</Select.Trigger>
					<Select.Content>
						<Select.Item value="-" on:click={() => seperatorChanged('-')}>Hyphen</Select.Item>
						<Select.Item value=" " on:click={() => seperatorChanged(' ')}>Space</Select.Item>
						<Select.Item value="." on:click={() => seperatorChanged('.')}>Periods</Select.Item>
						<Select.Item value="," on:click={() => seperatorChanged(',')}>Commas</Select.Item>
						<Select.Item value="_" on:click={() => seperatorChanged('_')}>Underscores</Select.Item>
					</Select.Content>
				</Select.Root>
			</div>
			<div class="flex items-center space-x-2">
				<Checkbox id="capitalise" bind:checked={capitaliseChecked} />
				<Label
					for="capitalise"
					class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
				>
					Capitalise
				</Label>
			</div>
		{:else}
			<div class="flex items-center space-x-2">
				<Select.Root>
					<Select.Label class="pl-0">Words</Select.Label>
					<Select.Trigger class="w-[180px]">
						<Select.Value placeholder="15" />
					</Select.Trigger>
					<Select.Content>
						<Select.Item value="3" on:click={() => bipWordsChanged(3)}>3</Select.Item>
						<Select.Item value="6" on:click={() => bipWordsChanged(6)}>6</Select.Item>
						<Select.Item value="9" on:click={() => bipWordsChanged(9)}>9</Select.Item>
						<Select.Item value="12" on:click={() => bipWordsChanged(12)}>12</Select.Item>
						<Select.Item value="15" on:click={() => bipWordsChanged(15)}>15</Select.Item>
						<Select.Item value="18" on:click={() => bipWordsChanged(18)}>18</Select.Item>
						<Select.Item value="21" on:click={() => bipWordsChanged(21)}>21</Select.Item>
						<Select.Item value="24" on:click={() => bipWordsChanged(24)}>24</Select.Item>
					</Select.Content>
				</Select.Root>
			</div>
		{/if}
	</div>
	<Button class="w-full" on:click={regenerate}>Regenerate</Button>
</div>
