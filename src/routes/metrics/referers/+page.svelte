<script lang="ts">
	import { onMount } from 'svelte';
	import { bb, donut } from 'billboard.js';
	import 'billboard.js/dist/theme/insight.css';

	export let data;

	const { referers } = data;

	onMount(() => {
		bb.generate({
			data: {
				columns: referers.map(({ referrer, count }: { referrer: string; count: number }) => [
					referrer,
					count,
				]),
				type: donut(),
			},
			bindto: '#chart',
		});
	});
</script>

<div id="chart" />
