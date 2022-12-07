<script lang="ts">
	import Note from '$lib/components/Note.svelte';
</script>

<div class="flex flex-row gap-4 pr-6 justify-end">
	<a href="/metrics/views" class="text-gray-600 text-sm">page views</a>
	<a href="/metrics/visited" class="text-gray-600 text-sm">most visited</a>
	<a href="/metrics/referers" class="text-gray-600 text-sm">referers</a>
</div>
<Note>
	<div class="pt-12">
		<slot />
	</div>
</Note>
