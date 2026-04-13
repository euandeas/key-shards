<script lang="ts">
	import * as Tooltip from './tooltip';

	export let text = '';
</script>

<Tooltip.Root openDelay={1000}>
	<Tooltip.Trigger>
		<slot />
	</Tooltip.Trigger>
	<Tooltip.Content>
		<p>{text}</p>
	</Tooltip.Content>
</Tooltip.Root>
