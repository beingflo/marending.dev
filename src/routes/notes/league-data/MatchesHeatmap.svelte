<script lang="ts">
	import Note from '$lib/components/Note.svelte';
	import Chart from '$lib/components/Chart.svelte';
	import * as Plot from '@observablehq/plot';
	import type { Match } from './utils';

	export let matches: Array<Match>;

	const matchesByDay = matches.reduce((prev: Record<string, number>, d) => {
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

	const matchFrequency = Object.entries(matchesByDay)?.map(([date, value]) => ({
		date: new Date(date),
		value,
	}));

	const getWeek = (date: Date): number => {
		const startDate = new Date(date.getFullYear(), 0, 1);
		const days = Math.floor((date.getTime() - startDate.getTime()) / (24 * 60 * 60 * 1000));

		return Math.ceil(days / 7);
	};
</script>

<Note>
	<div class="my-4">
		<Chart
			options={{
				x: { axis: null },
				y: {
					tickFormat: Plot.formatWeekday('en', 'narrow'),
					tickSize: 0,
				},
				fy: { tickFormat: '' },
				color: { scheme: 'Greens', type: 'log', legend: 'ramp' },
				marks: [
					Plot.cell(matchFrequency, {
						x: (d) => getWeek(d.date),
						y: (d) => d.date.getDay(),
						fill: 'value',
						fy: (d) => d.date.getFullYear(),
					}),
				],
			}} />
	</div>
</Note>
