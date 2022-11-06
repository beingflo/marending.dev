<script>
	import Note from '$lib/components/Note.svelte';
</script>

<div
	class="grid grid-cols-1 md:grid-cols-3 pt-12 md:pt-24 gap-8 md:gap-12 px-4 md:justify-items-center"
>
	<Note
		title="A study of lines"
		summary="Some schmuck tries his hands at art"
		date="06/11/2022"
		link="/lines"
	/>
	<Note
		title="Technical internals of this website"
		summary="There are many like it, but this one is mine"
		date="12/10/2022"
		link="/self"
	/>
	<Note
		title="Building a foosball ranking application"
		summary="How to waste time at work"
		date="04/09/2022"
		link="/foosball"
	/>
</div>
