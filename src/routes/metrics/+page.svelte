<script lang="ts">
	import Note from '$lib/components/Note.svelte';
	import { onMount } from 'svelte';
	import { bb, areaSpline } from 'billboard.js';
	import 'billboard.js/dist/theme/insight.css';

	export let data;

	const { pageViews } = data;

	onMount(() => {
		bb.generate({
			bindto: '#chart',
			data: {
				type: areaSpline(),
				x: 'day',
				columns: [
					['day', ...pageViews.map((pw: any) => pw.day)],
					['count', ...pageViews.map((pw: any) => pw.count)],
				],
			},
			axis: {
				x: {
					type: 'timeseries',
					tick: {
						format: '%Y-%m-%d',
						rotate: 50,
					},
				},
			},
			tooltip: {
				show: true,
			},
			point: {
				show: false,
			},
			legend: {
				hide: true,
			},
		});
	});
</script>

<Note>
	<div class="pt-12">
		<h1 class="text-lg pb-8">Page views</h1>
		<div id="chart" />
	</div>
</Note>
