<script lang="ts">
	import Chart from '$lib/components/Chart.svelte';
	import * as Plot from '@observablehq/plot';
	import type { Match } from './utils';
	import Figure from '$lib/components/lines/Figure.svelte';

	export let matches: Array<Match>;

	const matchesByDay = matches
		?.filter((m) => m.players.includes('fma'))
		.reduce((prev: Record<string, number>, d) => {
			const day = d.startDate.toLocaleString('en-US', {
				day: 'numeric',
				month: 'numeric',
				year: 'numeric',
			});

			if (prev[day]) {
				prev[day] += 1;
			} else {
				prev[day] = 1;
			}
			return prev;
		}, {});

	const matchFrequency = Object.entries(matchesByDay)?.map(([date, matches]) => ({
		date: new Date(date),
		matches,
	}));

	const getWeek = (date: Date): number => {
		const startDate = new Date(date.getFullYear(), 0, 1);
		const days = Math.floor((date.getTime() - startDate.getTime()) / (24 * 60 * 60 * 1000));

		return Math.ceil(days / 7);
	};
</script>

<Figure caption="Figure 2. Number of matches played per day by me">
	<Chart
		options={{
			x: { axis: null, domain: [...Array(52).keys()].map((x) => x + 1) },
			y: {
				tickFormat: Plot.formatWeekday('en', 'narrow'),
				tickSize: 0,
			},
			fy: { tickFormat: '' },
			color: { scheme: 'Greens', type: 'log', legend: 'ramp' },
			aspectRatio: 0.8,
			marks: [
				Plot.cell(matchFrequency, {
					x: (d) => getWeek(d.date),
					y: (d) => d.date.getDay(),
					fill: 'matches',
					fy: (d) => d.date.getFullYear(),
				}),
			],
		}} />
</Figure>
