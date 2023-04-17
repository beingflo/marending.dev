<script>
	import NoteSummary from '$lib/components/NoteSummary.svelte';
	import WidePage from '$lib/components/WidePage.svelte';
</script>

<WidePage>
	<div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-8 md:gap-12 md:gap-y-16">
		<NoteSummary
			title="A comparison of wireless protocols for battery powered embedded devices"
			summary="Trying to sleep as much as possible"
			date="17 April 2023"
			link="/notes/esp-protocol" />
		<NoteSummary
			title="Measuring power consumption of ESP32 development boards"
			summary="First forays into embedded devices"
			date="25 February 2023"
			link="/notes/esp-power" />
		<NoteSummary
			title="Performance tuning a specific SvelteKit application"
			summary="Making sure my website can handle all those page views it isn't getting"
			date="29 December 2022"
			link="/notes/performance" />
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
</WidePage>
