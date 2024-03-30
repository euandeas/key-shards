<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';

	export let secret: string = '';
	let secretasutf = '';

	$: {
		trymnemonic(secret);
	}

	async function trymnemonic(str: string) {
		secretasutf = await invoke('tryutf8tomnemonic', { utf: str });
	}
</script>

<div class="mt-12 flex h-5/6 items-center justify-center">
	<div class="flex flex-col items-center">
		<h3 class="mx-auto mb-5 max-w-lg scroll-m-20 text-center text-2xl font-semibold tracking-tight">
			{secret}
		</h3>
		<small class="mb-10 text-center text-sm font-medium leading-none text-gray-500">
			{secretasutf}
		</small>
		<small class="text-center text-sm font-medium leading-none text-gray-500">
			Displaying a secret does not ensure its validity. Only the XChaCha20-Poly1305 option can
			provide this guarantee.
		</small>
	</div>
</div>
