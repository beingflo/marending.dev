<script lang="ts">
	import Chart from '$lib/components/Chart.svelte';
	import * as Plot from '@observablehq/plot';
	import type { Match } from './utils';
	import Figure from '$lib/components/lines/Figure.svelte';
	import A from '$lib/components/A.svelte';

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

<Figure
	caption="Figure 5. Matches versus goals per match for players who scored at least 100 goals (mention game mode, 10 games different)">
	<Chart
		options={{
			x: { grid: true },
			y: { grid: true, label: '↑ goals / match' },
			marginLeft: 50,
			marks: [
				Plot.dot(
					data.filter((d) => d.goals > 100),
					{
						x: 'matches',
						y: (d) => d.goals / d.matches,
						stroke: (d) => (d.player === 'fma' ? 'red' : 'black'),
					}
				),
				Plot.text(
					data.filter((d) => d.goals > 100),
					{
						x: 'matches',
						y: (d) => d.goals / d.matches,
						text: (d) => '← me',
						filter: (d) => d.player === 'fma',
						dy: 3.5,
						dx: 20,
						lineAnchor: 'bottom',
					}
				),
			],
		}} />
</Figure>
