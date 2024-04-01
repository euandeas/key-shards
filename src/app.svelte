<script lang="ts">
	import { Toaster } from './lib/components/sonner';
	import Create from './pages/create.svelte';
	import Combine from './pages/combine.svelte';
	import AppRail from './lib/components/apprail/apprail.svelte';
	import AppRailTile from './lib/components/apprail/apprailtile.svelte';
	import SlideUp from './lib/components/slideup.svelte';
	import Info from './pages/info.svelte';
	import Password from './pages/password.svelte';

	if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
		document.documentElement.classList.add('dark');
	} else {
		document.documentElement.classList.remove('dark');
	}

	let currentPage: number = 0;
	let showNewPage: boolean = false;
</script>

<main class="relative h-full overflow-y-hidden">
	<Toaster />
	<div class="flex h-screen">
		<AppRail class="h-screen">
			<AppRailTile bind:group={currentPage} name="create-tile" value={0} title="Create Shares">
				<svelte:fragment slot="lead">(icon)</svelte:fragment>
				<span>Create</span>
			</AppRailTile>
			<AppRailTile bind:group={currentPage} name="combine-tile" value={1} title="Combine Shares">
				<svelte:fragment slot="lead">(icon)</svelte:fragment>
				<span>Combine</span>
			</AppRailTile>
			<AppRailTile
				bind:group={currentPage}
				name="password-tile"
				value={2}
				title="Password Generator"
			>
				<svelte:fragment slot="lead">(icon)</svelte:fragment>
				<span>Generator</span>
			</AppRailTile>
			<svelte:fragment slot="trail">
				<AppRailTile bind:group={showNewPage} name="info-tile" value={true} title="Info">
					(info icon)
				</AppRailTile>
			</svelte:fragment>
		</AppRail>

		<div class="flex-1">
			{#if currentPage === 0}
				<Create />
			{:else if currentPage === 1}
				<Combine />
			{:else}
				<Password />
			{/if}
		</div>
	</div>
	<SlideUp bind:showNewPage>
		<Info />
	</SlideUp>
</main>
