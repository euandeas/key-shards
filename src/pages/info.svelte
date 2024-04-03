<script lang="ts">
	import * as Accordion from '../lib/components/accordion';
	import AccordionItem from '../lib/components/accordionitem.svelte';
</script>

<div class="mt-12 flex h-5/6 items-center justify-center">
	<Accordion.Root class="w-5/6">
		<AccordionItem value="whatis" title="About">
			<p>
				Key Shards is a distributed backup application for private keys, passwords or any other
				important information. A number of shares are produced and only a subset of them are
				required to reveal the secret.<br /><br />

				Unlike conventional secret sharing this is not targetted towards distributing a secret among
				a group, but instead the personal back up of secrets. However this application can be used
				for this convential purpose if you understand what you are doing and the surrounding
				security risks.<br /><br />

				This is based on an implementation of
				<a href="https://en.wikipedia.org/wiki/Shamir's_secret_sharing">Shamir's Secret Sharing</a>.
			</p>
		</AccordionItem>
		<AccordionItem value="example" title="Example Use Case">
			<p></p>
		</AccordionItem>
		<AccordionItem value="creation" title="Creating Shares">
			<Accordion.Root class="ml-5">
				<AccordionItem value="creation-1" title="Basics">
					<p></p>
				</AccordionItem>
				<AccordionItem value="creation-2" title="Advanced Option 1: Predefined Shares">
					<p></p>
				</AccordionItem>
				<AccordionItem value="creation-3" title="Advanced Option 2: XChaCha20-Poly1305 AEAD">
					<p></p>
				</AccordionItem>
				<AccordionItem value="creation-4" title="Advanced Option 3: Padding">
					<p></p>
				</AccordionItem>
				<AccordionItem value="creation-5" title="Storing Shares">
					<p></p>
				</AccordionItem>
				<AccordionItem value="creation-5" title="Technical Details">
					<p></p>
				</AccordionItem>
			</Accordion.Root>
		</AccordionItem>
		<AccordionItem value="combination" title="Combining Shares">
			<p></p>
		</AccordionItem>
		<AccordionItem value="password" title="Generating Passwords">
			<AccordionItem value="Passwords" title="Passwords">
				<p></p>
			</AccordionItem>
			<AccordionItem value="Passphrases" title="Passphrases">
				<p></p>
			</AccordionItem>
			<AccordionItem value="BIP Mnemonic" title="BIP Mnemonic">
				<p></p>
			</AccordionItem>
		</AccordionItem>
	</Accordion.Root>
</div>

<style>
	a {
		text-decoration: underline;
	}
</style>
