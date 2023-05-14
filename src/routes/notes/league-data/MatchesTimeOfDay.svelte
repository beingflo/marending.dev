<script lang="ts">
	import Chart from '$lib/components/Chart.svelte';
	import * as Plot from '@observablehq/plot';
	import type { Match } from './utils';
	import Figure from '$lib/components/lines/Figure.svelte';

	export let matches: Array<Match>;

	const hours: Record<number, number> = {};

	matches.forEach((m) => {
		const hour = m.startDate.getHours();
		if (hours[hour]) {
			hours[hour] += 1;
		} else {
			hours[hour] = 1;
		}
	});

	const data = Object.entries(hours).map(([hour, matches]) => ({
		hour: Number(hour),
		matches,
	}));
	data.sort((a, b) => a.hour - b.hour);
</script>

<Figure caption="Figure 2. Time of day distribution of all games">
	<Chart
		options={{
			y: { grid: true, label: '↑ matches' },
			x: { label: 'hour' },
			marks: [Plot.barY(data, { x: 'hour', y: 'matches' })],
		}} />
</Figure>
