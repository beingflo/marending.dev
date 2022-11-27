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

<div class="p-2 md:p-8 grid grid-cols-1 md:grid-cols-2">
	<div>
		<h1 class="text-lg">Page views</h1>
		<div class="">
			<Chart
				data={lineChart}
				type="line"
				lineOptions={{ spline: 1, hideDots: 1, regionFill: 1 }}
				axisOptions={{ yAxisMode: 'tick', xAxisMode: 'tick', xIsSeries: 1 }} />
		</div>
	</div>
	<div>
		<h1 class="text-lg pt-8 md:pt-0">Most viewed</h1>
		<Chart data={pieChart} type="pie" maxSlices={8} />
	</div>
</div>
