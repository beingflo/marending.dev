<script>
	// @ts-nocheck
	import Note from '$lib/components/Note.svelte';
	import Chart from 'svelte-frappe-charts';

	export let data;

	const { pageViews, mostViewed } = data;

	const lineChart = {
		labels: pageViews.map((view) => view.day),
		datasets: [
			{
				values: pageViews.map((view) => view.count),
			},
		],
	};

	const pieChart = {
		labels: mostViewed.map((page) => page.page),
		datasets: [
			{
				values: mostViewed.map((page) => page.count),
			},
		],
	};
</script>

<Note>
	<h1 class="text-lg">Page views</h1>
	<div class="">
		<Chart
			data={lineChart}
			type="line"
			lineOptions={{ spline: 1, hideDots: 1, regionFill: 1 }}
			axisOptions={{ yAxisMode: 'tick', xAxisMode: 'tick', xIsSeries: 1 }} />
	</div>
	<h1 class="text-lg pt-16">Most viewed</h1>
	<Chart data={pieChart} type="pie" maxSlices={8} />
</Note>
