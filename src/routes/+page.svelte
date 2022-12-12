<script>
	import NoteSummary from '$lib/components/NoteSummary.svelte';
	import Page from '$lib/components/Page.svelte';
</script>

<Page>
	<div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-8 md:gap-12">
		<NoteSummary
			title="A study of lines"
			summary="Some schmuck tries his hands at generative art"
			date="03 December 2022"
			link="/notes/lines" />
		<NoteSummary
			title="About this website"
			summary="There are many like it, but this one is mine"
			date="27 November 2022"
			link="/notes/website" />
	</div>
</Page>
