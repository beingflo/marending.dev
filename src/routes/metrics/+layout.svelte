<script lang="ts">
	import Note from '$lib/components/Note.svelte';
	import { page } from '$app/stores';
</script>

<div class="flex flex-row gap-4 pr-6 justify-end">
	<a
		href="/metrics/views"
		class="text-gray-600 text-sm {$page.url.pathname === '/metrics/views' && 'underline'}"
		>page views</a>
	<a
		href="/metrics/visited"
		class="text-gray-600 text-sm  {$page.url.pathname === '/metrics/visited' && 'underline'}"
		>most visited</a>
	<a
		href="/metrics/referers"
		class="text-gray-600 text-sm  {$page.url.pathname === '/metrics/referers' && 'underline'}"
		>referers</a>
</div>
<Note>
	<div class="pt-12">
		<slot />
	</div>
</Note>
