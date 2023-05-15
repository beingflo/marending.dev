<script lang="ts">
	import Chart from '$lib/components/Chart.svelte';
	import * as Plot from '@observablehq/plot';
	import type { Match } from './utils';
	import Figure from '$lib/components/lines/Figure.svelte';

	export let matches: Array<Match>;

	const filteredMatches = matches.filter(
		(m) => (m.endDate.getTime() - m.startDate.getTime()) / 60000 < 20
	);
</script>

<Figure caption="Figure 2. Moving average of the duration of games">
	<Chart
		options={{
			y: { grid: true, label: '↑ duration [m]' },
			x: { label: 'time →' },
			marks: [
				Plot.lineY(
					filteredMatches,
					Plot.windowY(500, {
						x: 'startDate',
						y: (d) => (d.endDate.getTime() - d.startDate.getTime()) / 60000,
					})
				),
			],
		}} />
</Figure>
