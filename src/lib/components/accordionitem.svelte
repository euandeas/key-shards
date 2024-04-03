<script lang="ts">
	import * as Accordion from './accordion';

	export let value = '';
	export let title = '';
</script>

<div>
	<Accordion.Item {value}>
		<Accordion.Trigger>{title}</Accordion.Trigger>
		<Accordion.Content>
			<slot />
		</Accordion.Content>
	</Accordion.Item>
</div>
