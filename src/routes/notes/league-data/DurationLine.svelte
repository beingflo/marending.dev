<script lang="ts">
	import Chart from '$lib/components/Chart.svelte';
	import * as Plot from '@observablehq/plot';
	import type { Match } from './utils';
	import Figure from '$lib/components/lines/Figure.svelte';

	export let matches: Array<Match>;
	console.log(
		matches.reduce(
			(ag, m) => {
				const minutes = (m.endDate.getTime() - m.startDate.getTime()) / 60000;
				let newAg = ag;
				if (minutes <= 20) {
					newAg[0] += minutes;
					newAg[1] += 1;
				}
				return newAg;
			},
			[0, 0]
		)
	);
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
