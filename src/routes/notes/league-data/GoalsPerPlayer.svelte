<script lang="ts">
	import Chart from '$lib/components/Chart.svelte';
	import * as Plot from '@observablehq/plot';
	import type { Match } from './utils';
	import Figure from '$lib/components/lines/Figure.svelte';

	export let matches: Array<Match>;

	const goalsPerPlayer: Record<string, number> = {};

	matches.forEach((m) => {
		m.goals?.scorer.forEach((p) => {
			if (goalsPerPlayer[p]) {
				goalsPerPlayer[p] += 1;
			} else {
				goalsPerPlayer[p] = 1;
			}
		});
	});

	const data = Object.entries(goalsPerPlayer).map(([p, n]) => ({ player: p, goals: n }));
	data.sort((a, b) => b.goals - a.goals);
</script>

<Figure caption="Figure 3. Number of goals scored by player. Limited to the top 100 players">
	<Chart
		options={{
			x: { tickSize: 0, tickFormat: (d) => '', label: 'Players' },
			y: { grid: true },
			marginLeft: 50,
			marks: [
				Plot.barY(data.slice(0, 100), {
					x: 'player',
					y: 'goals',
					fill: (d) => (d.player === 'fma' ? 'red' : 'black'),
					sort: { x: 'y', reverse: true },
				}),
				Plot.text(data.slice(0, 100), {
					x: 'player',
					y: 'goals',
					text: (d) => '← this is me',
					filter: (d) => d.player === 'fma',
					rotate: -90,
					dy: -40,
					dx: 3.5,
					lineAnchor: 'bottom',
				}),
			],
		}} />
</Figure>
