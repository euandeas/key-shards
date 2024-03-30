<script lang="ts">
	import { Toaster } from './lib/components/sonner';
	import Create from './lib/pages/create.svelte';
	import Combine from './lib/pages/combine.svelte';
	import AppRail from './lib/components/apprail/apprail.svelte';
	import AppRailTile from './lib/components/apprail/apprailtile.svelte';
	import AppRailAnchor from './lib/components/apprail/apprailanchor.svelte';

	if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
		document.documentElement.classList.add('dark');
	} else {
		document.documentElement.classList.remove('dark');
	}

	let currentPage: number = 0;
</script>

<main>
	<Toaster />
	<div class="flex h-screen">
		<AppRail class="h-screen">
			<!-- --- -->
			<AppRailTile bind:group={currentPage} name="tile-1" value={0} title="tile-1">
				<svelte:fragment slot="lead">(icon)</svelte:fragment>
				<span>Create</span>
			</AppRailTile>
			<AppRailTile bind:group={currentPage} name="tile-2" value={1} title="tile-2">
				<svelte:fragment slot="lead">(icon)</svelte:fragment>
				<span>Combine</span>
			</AppRailTile>
			<!-- --- -->
			<svelte:fragment slot="trail">
				<AppRailAnchor href="/" target="_blank" title="Account">(info icon)</AppRailAnchor>
			</svelte:fragment>
		</AppRail>

		<div class="flex-1">
			{#if currentPage === 0}
				<Create />
			{:else}
				<Combine />
			{/if}
		</div>
	</div>
</main>
