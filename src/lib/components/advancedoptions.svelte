<script lang="ts">
    import { Checkbox } from '../components/checkbox';
    import { Label } from '../components/label';
  
    export let checkedpre: boolean;
    export let checkedaead: boolean;
    export let checkedpadding: boolean;
  </script>
  
  <div class="mb-8 flex">
    <div class="mr-6 flex items-center space-x-2">
      <Checkbox id="predefined" bind:checked={checkedpre} />
      <Label
        for="predefined"
        class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
      >
        Predefined Shares
      </Label>
    </div>
    <div class="mr-6 flex items-center space-x-2">
      <Checkbox id="aead" bind:checked={checkedaead} />
      <Label
        for="aead"
        class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
      >
        XChaCha20-Poly1305 AEAD
      </Label>
    </div>
    <div class="flex items-center space-x-2">
      <Checkbox id="padding" bind:checked={checkedpadding} />
      <Label
        for="padding"
        class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
      >
        Padding
      </Label>
    </div>
  </div>