<script lang="ts">
    import * as Accordion from "../lib/components/accordion";
</script>

<div class="mt-12 flex h-5/6 items-center justify-center">
    <Accordion.Root class="w-3/4">
        <Accordion.Item value="item-1">
          <Accordion.Trigger>Is it accessible?</Accordion.Trigger>
          <Accordion.Content>
            Yes. It adheres to the WAI-ARIA design pattern.
          </Accordion.Content>
        </Accordion.Item>
      </Accordion.Root>
</div>
