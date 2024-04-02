<script lang="ts">
	import Exit from '../icons/exit.svelte';

	export let showNewPage: boolean;
</script>

<div
	class={`z-1 absolute overflow-x-hidden transition-all duration-1000 ease-in-out ${
		showNewPage ? 'translate-y-0' : 'translate-y-full'
	} top-0 h-full w-full bg-background p-8`}
>
	<button
		class="absolute right-0 top-0 m-6 text-gray-500 hover:text-gray-700 focus:outline-none"
		on:click={() => (showNewPage = false)}
	>
		<Exit />
	</button>
	<slot />
</div>
