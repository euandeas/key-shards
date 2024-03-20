<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';

	export let results: string[] = [];
</script>

<div>
	<ul>
		{#each results as result}
			<li>{result}</li>
		{/each}
	</ul>
</div>
