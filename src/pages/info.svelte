<script lang="ts">
	import * as Accordion from '../lib/components/accordion';
	import AccordionItem from '../lib/components/accordionitem.svelte';
</script>

<div class="mt-12 flex h-5/6 items-center justify-center">
	<Accordion.Root class="w-5/6">
		<AccordionItem value="whatis" title="About">
			<p>
				Key Shards is a distributed backup application for private keys, passwords or any other
				important information. Several shares are produced and only a subset of them are required to
				reveal the secret.<br /><br />

				This program would not be suitable for backing up every password that you use, a password
				manager is more suited for this job. However, this program is suited for backing up master
				passwords or the secret key that some services give you (e.g. 1Password or Proton).
				Other use cases include cryptocurrency wallet keys or succession planning.<br /><br />

				Unlike conventional secret sharing, this is not targeted towards distributing a secret among
				a group, but instead the personal backup of secrets. However, this application can be used
				for this conventional purpose if you understand what you are doing and the surrounding
				security risks.<br /><br />

				This is based on an implementation of
				<a href="https://en.wikipedia.org/wiki/Shamir's_secret_sharing">Shamir's Secret Sharing</a>.
			</p>
		</AccordionItem>
		<AccordionItem value="creation" title="Creating Shares">
			<Accordion.Root class="ml-5">
				<AccordionItem value="creation-1" title="Basics">
					<p>
						Start by entering your secret, the minimum threshold and the total shares. The minimum
						threshold is the number of shares that are required to reveal the secret. The total
						shares are the number of shares that will be produced.<br /><br />

						The basic recommended settings are:<br />
						&nbsp;&nbsp;&nbsp;Threshold:&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;3<br />
						&nbsp;&nbsp;&nbsp;Total Shares: 5<br /><br />

						You can copy or export each share in various representations and formats. The shares can
						be encoded into two text formats, either Base64 or BIP Mnemonic. Shares can also be
						exported as a file or QR code.<br /><br />

						It is important to note that nothing about your secret is revealed by the shares unless
						you bring the correct amount of shares together (except the length, this can be
						mitigated with padding).<br /><br />

						When creating shares several different advanced options can be used. It is not
						recommended to modify these unless you understand the security consequences they may
						have.
					</p>
				</AccordionItem>
				<AccordionItem value="creation-2" title="Advanced Option 1: Predefined Shares">
					<p>
						<b>Default: Off</b><br /><br />
						<b
							>This option is experimental the security consequences have not been fully
							investigated, use with caution!</b
						><br /><br />

						When this option is selected you will be able to generate up to two short memorable
						phrases, that will act as normal shares. These should never be stored and are only to be
						memorised. When generating with this option selected you must have at least one more
						share for your threshold.
					</p>
				</AccordionItem>
				<AccordionItem value="creation-3" title="Advanced Option 2: XChaCha20-Poly1305 AEAD">
					<p>
						<b>Default: On</b><br /><br />
						When this option is selected the secret is first encrypted using the XChaCha20-Poly1305 scheme,
						and then the key of this scheme is passed through the secret sharing algorithm.
						<br /><br />

						This has several benefits. It not only prevents
						<a href="https://en.wikipedia.org/wiki/Side-channel_attack">Side Channel Attacks</a> but
						it also verifies the integrity of the secret upon decryption. This means that you can be
						sure that the secret revealed to you is valid and the original one you entered.
					</p>
				</AccordionItem>
				<AccordionItem value="creation-4" title="Advanced Option 3: Padding">
					<p>
						<b>Default: On</b><br /><br />

						This option adds padding to the secret so that the shares are multiples of 8 bytes. This
						is useful for hiding the length of the original secret, which in some cases could be
						considered leaked information about the secret.
					</p></AccordionItem
				>
			</Accordion.Root>
		</AccordionItem>
		<AccordionItem value="creation-5" title="Storing Shares">
			<p>
				The strategy for distributing and storing shares is to keep them all in completely unrelated
				places. This means both digitally and physically. As an example, it would be bad practice to
				store shares on both your phone and in your wallet if you always carry them around with each
				other. You should always have as few of the shares as easy to access and keep the rest in
				harder-to-access places (e.g. for a threshold of three only one should be easy to access).<br
				/><br />

				Physically you could note down the share or export it and print the QR code. Examples of
				potential places to keep physical shares are in your wallet or hidden at home.<br /><br />

				Digitally you could export the file or QR code. These can be stored on your personal
				devices, in cloud services or storage devices (e.g. USB or external hard drives).<br /><br
				/>
			</p>
		</AccordionItem>
		<AccordionItem value="combination" title="Accessing Secret">
			<p>
				When you want to access a secret, all you must do is enter all the shares you have access
				to, until you meet the threshold requirement. The order of the shares does not matter and
				the program will automatically handle the different options if you selected any when
				creating shares.<br /><br />

				You have many options for entering the shares. You can enter the text as it is, or upload
				the file or QR code. If your device has a camera you may also be able to scan your QR code
				using your device. If you can't upload or scan the QR code to the device then it is possible
				to scan it with another and copy the data as shown.<br /><br />

				Unless you selected the XChaCha20-Poly1305 option, there is no guarantee that the secret
				shown to you is the right one if you may have entered the wrong amount of shares or the
				wrong shares.
			</p>
		</AccordionItem>
		<AccordionItem value="password" title="Password Advice">
			<p><b>Strong Passwords</b></p>
			<br />
			<p>
				Strong passwords have high <a
					href="https://en.wikipedia.org/wiki/Entropy_(information_theory)">entropy</a
				>. Entropy is a measure of how unpredictable your password is, by following the below advice
				you can make your passwords more unpredictable:<br /><br />
			</p>
			<p class="ml-5">
				1. <b>Length</b>: Aim for at least 12 to 15 characters, the longer the password the stronger
				it will be.
				<br /><br />
				2. <b>Complexity</b>: Use a combination of uppercase letters, lowercase letters, numbers,
				and symbols.
				<br /><br />
				3. <b>Avoid Predictability</b>: Don't use easily guessed passwords like “123456”,
				“password”, or “qwerty”.
				<br /><br />
				4. <b>Avoid Personal Information</b>: Dont use birthdays, names of your pets or loved ones,
				or anniversaries.
				<br /><br />
				5. <b>Uniqueness</b>: Don't reuse passwords between different accounts. If you reuse a
				password across multiple services, then a data breach on one service can result in hackers
				gaining access to all the other services secured with the same password.
				<br /><br />
			</p>

			<p><b>Passphrases</b></p>
			<br />
			<p>
				If you need to remember passwords or want them easier to write/type out, passphrases are a
				good alternative. In this program, you have to option of generating either a passphrase use
				the
				<a href="https://www.eff.org/files/2016/09/08/eff_short_wordlist_1.txt"
					>EFF Short Word List</a
				>
				or a BIP Mnemonic. The difference between them is the word lists they use. BIP Mnemonics
				have the benefit that they can be compressed by the secret sharing implementation used in
				this program, creating shorter shares.<br /><br />
			</p>
			<p><b>Storing Passwords</b></p>
			<br />
			<p>
				By creating unique passwords for each service that you use, you can be left with far too
				many passwords to remember. The best solution to this is to use a password manager. You must
				create a complex but memorable master password. This program can then be used to back up
				this master password storing shares in secure locations.<br /><br />
			</p>
		</AccordionItem>
	</Accordion.Root>
</div>

<style>
	a {
		text-decoration: underline;
	}
</style>
