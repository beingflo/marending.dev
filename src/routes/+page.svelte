<script>
	import NoteSummary from '$lib/components/NoteSummary.svelte';
</script>

<div
	class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 pt-8 md:pt-24 gap-8 md:gap-12 px-4 md:justify-items-center"
>
	<NoteSummary
		title="A study of lines"
		summary="Some schmuck tries his hands at generative art"
		date="06/11/2022"
		link="/lines"
	/>
	<NoteSummary
		title="Technical internals of this website"
		summary="There are many like it, but this one is mine"
		date="12/10/2022"
		link="/self"
	/>
	<NoteSummary
		title="Building a foosball ranking application"
		summary="How to waste time at work"
		date="04/09/2022"
		link="/foosball"
	/>
	<NoteSummary
		title="A hasslefree server setup for personal projects"
		summary="Don't spend more time on infra than on your projects"
		date="10/11/2022"
		link="/server"
	/>
</div>
