<script lang="ts">
	import Note from '$lib/components/Note.svelte';
</script>

<Note>
	<div class="pt-12">
		<slot />
	</div>
	<div class="flex flex-row gap-4">
		<a href="/metrics/views" class="text-gray-800 text-sm">page views</a>
		<a href="/metrics/visited" class="text-gray-800 text-sm">most visited</a>
		<a href="/metrics/referers" class="text-gray-800 text-sm">referers</a>
	</div>
</Note>
