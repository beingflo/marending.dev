<script lang="ts">
	import Chart from '$lib/components/Chart.svelte';
	import * as Plot from '@observablehq/plot';
	import type { Match } from './utils';
	import Figure from '$lib/components/lines/Figure.svelte';

	export let matches: Array<Match>;

	const days: Record<number, number> = {};

	matches.forEach((m) => {
		const day = m.startDate.getDay();
		if (days[day]) {
			days[day] += 1;
		} else {
			days[day] = 1;
		}
	});

	const data = Object.entries(days).map(([day, matches]) => ({
		day: Number(day),
		matches,
	}));
	data.sort((a, b) => a.day - b.day);
</script>

<Figure caption="Figure 3. Day of week distribution of all games">
	<Chart
		options={{
			y: { grid: true, label: '↑ matches' },
			x: { label: 'day', tickFormat: Plot.formatWeekday('en-US', 'short') },
			marks: [Plot.barY(data, { x: 'day', y: 'matches' })],
		}} />
</Figure>
