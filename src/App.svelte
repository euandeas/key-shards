<script lang="ts">
  import Greet from './lib/Greet.svelte'
  import AppRail from './lib/components/AppRail/AppRail.svelte'
  import AppRailTile from './lib/components/AppRail/AppRailTile.svelte'
  import AppRailAnchor from './lib/components/AppRail/AppRailAnchor.svelte'

  if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
    document.documentElement.classList.add('dark')
  } else {
    document.documentElement.classList.remove('dark')
  }

  let currentTile: number = 0;
</script>

<main>
  <AppRail class="h-screen">
    <!-- --- -->
    <AppRailTile bind:group={currentTile} name="tile-1" value={0} title="tile-1">
      <svelte:fragment slot="lead">(icon)</svelte:fragment>
      <span>Tile 1</span>
    </AppRailTile>
    <AppRailTile bind:group={currentTile} name="tile-2" value={1} title="tile-2">
      <svelte:fragment slot="lead">(icon)</svelte:fragment>
      <span>Tile 2</span>
    </AppRailTile>
    <AppRailTile bind:group={currentTile} name="tile-3" value={2} title="tile-3">
      <svelte:fragment slot="lead">(icon)</svelte:fragment>
      <span>Tile 3</span>
    </AppRailTile>
    <!-- --- -->
    <svelte:fragment slot="trail">
      <AppRailAnchor href="/" target="_blank" title="Account">(icon)</AppRailAnchor>
    </svelte:fragment>
  </AppRail>
</main>