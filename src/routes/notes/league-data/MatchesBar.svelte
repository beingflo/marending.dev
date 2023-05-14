<script lang="ts">
	import Chart from '$lib/components/Chart.svelte';
	import * as Plot from '@observablehq/plot';
	import type { Match } from './utils';
	import Figure from '$lib/components/lines/Figure.svelte';

	export let matches: Array<Match>;
</script>

<Figure caption="Figure 2. Number of matches played per quarter">
	<Chart
		options={{
			y: { grid: true, label: '↑ matches' },
			x: { label: 'time →' },
			marks: [
				Plot.rectY(matches, Plot.binX({ y: 'count', interval: 'quarter' }, { x: 'startDate' })),
			],
		}} />
</Figure>
