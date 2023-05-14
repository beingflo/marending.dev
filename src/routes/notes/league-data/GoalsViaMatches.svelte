<script lang="ts">
	import Chart from '$lib/components/Chart.svelte';
	import * as Plot from '@observablehq/plot';
	import type { Match } from './utils';
	import Figure from '$lib/components/lines/Figure.svelte';

	export let matches: Array<Match>;

	const players: Record<string, [number, number]> = {};

	matches.forEach((m) => {
		m.goals?.scorer.forEach((p) => {
			if (players[p]) {
				players[p][0] += 1;
			} else {
				players[p] = [1, 0];
			}
		});
	});

	matches.forEach((m) => {
		m.players.forEach((p) => {
			if (players[p]) {
				players[p][1] += 1;
			} else {
				players[p] = [0, 1];
			}
		});
	});

	const data = Object.entries(players).map(([player, [goals, matches]]) => ({
		player,
		goals,
		matches,
	}));
</script>

<Figure caption="Figure 3. Number of goals scored by player. Limited to the top 100 players">
	<Chart
		options={{
			x: { grid: true },
			y: { grid: true },
			marginLeft: 50,
			marks: [
				Plot.dot(
					data.filter((d) => d.goals > 100),
					{ x: 'matches', y: 'goals', stroke: (d) => (d.player === 'fma' ? 'orange' : 'black') }
				),
				Plot.linearRegressionY(
					data.filter((d) => d.goals > 100),
					{ x: 'matches', y: 'goals', stroke: 'steelblue' }
				),
				Plot.text(
					data.filter((d) => d.goals > 100),
					{
						x: 'matches',
						y: 'goals',
						text: (d) => '← yours truly',
						filter: (d) => d.player === 'fma',
						rotate: -90,
						dy: -40,
						dx: 3.5,
						lineAnchor: 'bottom',
					}
				),
			],
		}} />
</Figure>
