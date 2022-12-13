<script lang="ts">
	import A from '$lib/components/A.svelte';
	import Page from '$lib/components/Page.svelte';
</script>

<Page>
	<div class="grid grid-cols-1 md:grid-cols-3 gap-y-2 md:gap-y-24">
		<A href="https://marending.dev">marending.dev</A>
		<span class="col-span-2 mb-8 md:mb-0"
			>My personal website. Custom designed and built with Sveltekit. It features a metrics
			dashboard that displays page views, referers and commonly accessed routes. Deployed on a small
			VPS using Caddy as a reverse proxy.
		</span>
		<A href="https://rest.quest">rest.quest</A>
		<span class="col-span-2 mb-8 md:mb-0">
			A minimal and opinionated todo web application. The user interface is designed to remove
			friction when adding projects or tasks. The state is primarily stored locally in the browser.
			Optionally, the user can supply object storage credentials to synchronize the state across
			multiple browsers. The application is implemented using SolidJS to keep the bundle size lean.
		</span>
		<A href="https://write.fieldnotes.land">fieldnotes.land</A>
		<span class="col-span-2 mb-8 md:mb-0">
			An end-to-end encrypted note taking application. The web application features a <em>
				wysiwyg</em> markdown editor and advanced formatting including tables. Content is encrypted using
			a private key that is only stored in the browser. By encoding the decryption key in the hash of
			the URL, it's possible to share notes without the server ever seeing the data in plain text. The
			backend stores encrypted content in PostgreSQL.
		</span>
	</div>
</Page>
