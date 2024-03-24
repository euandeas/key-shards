<script lang="ts">
	import { Button } from '../components/ui/button';
	import { Input } from '../components/ui/input';
	import { invoke } from '@tauri-apps/api/tauri';


	export let results: string[] = [];
	export let numofpredefined: number = 0;
</script>

<div class="mt-12">
	{#each results as result}
		<div class="mb-6">
			<Input value="{result}" disabled class="select-text disabled:cursor-text disabled:opacity-100 mb-2"/>
			<div class="flex w-full">
				<Button class="w-full py-2 px-4 rounded-e-none">Generate</Button>
				<Button class="w-full py-2 px-4 border-x border-grey rounded-none">Edit</Button>
				<Button class="w-full py-2 px-4 rounded-s-none">Delete</Button>
			  </div>
		</div>
	{/each}
</div>
