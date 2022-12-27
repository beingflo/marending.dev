<script lang="ts">
	import { page } from '$app/stores';
	import Page from '$lib/components/Page.svelte';
</script>

<div class="flex flex-row gap-4 pr-6 justify-end">
	<a href="/metrics/views" class="text-sm {$page.url.pathname === '/metrics/views/' && 'underline'}"
		>page views</a>
	<a
		href="/metrics/visited"
		class="text-sm  {$page.url.pathname === '/metrics/visited/' && 'underline'}">most visited</a>
	<a
		href="/metrics/referers"
		class="text-sm  {$page.url.pathname === '/metrics/referers/' && 'underline'}">referers</a>
</div>
<Page>
	<div>
		<slot />
	</div>
</Page>
