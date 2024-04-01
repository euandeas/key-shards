<script lang="ts">
	import { toast } from 'svelte-sonner';
	import { Label } from '../lib/components/label';
	import { Textarea } from '../lib/components/textarea';
	import { Input } from '../lib/components/input';
	import { Checkbox } from '../lib/components/checkbox';
	import { Button } from '../lib/components/button';
	import AdvancedOptions from '../lib/components/advancedoptions.svelte';
	import PredefinedShares from '../lib/components/predefinedshares.svelte';

	import { invoke } from '@tauri-apps/api/tauri';

</script>

<div class="relative h-full overflow-y-hidden">
	<div class="z-0 h-full select-none p-8">
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
				<Checkbox id="advanced"/>
				<Label
					for="advanced"
					class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
				>
					Advanced
				</Label>
			</div>
		</div>
	</div>
</div>
