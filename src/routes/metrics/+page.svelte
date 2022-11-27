<script>
	// @ts-nocheck
	import Note from '$lib/components/Note.svelte';
	import Chart from 'svelte-frappe-charts';

	export let data;

	const { pageViews } = data;

	const chartData = {
		labels: pageViews.map((view) => view.day),
		datasets: [
			{
				values: pageViews.map((view) => view.count),
			},
		],
	};
</script>

<Note>
	<h1 class="text-lg ml-8">Page views</h1>
	<Chart data={chartData} type="line" lineOptions={{ spline: 1, hideDots: 1, regionFill: 1 }} />
</Note>
