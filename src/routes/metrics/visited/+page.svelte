<script lang="ts">
	import { onMount } from 'svelte';
	import { bb, donut } from 'billboard.js';
	import 'billboard.js/dist/theme/insight.css';

	export let data;

	const { mostVisited } = data;

	onMount(() => {
		bb.generate({
			data: {
				columns: mostVisited.map(({ page, count }: { page: string; count: number }) => [
					page,
					count,
				]),
				type: donut(),
			},
			bindto: '#chart',
		});
	});
</script>

<div id="chart" />
