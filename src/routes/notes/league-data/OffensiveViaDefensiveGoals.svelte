<script lang="ts">
	import Chart from '$lib/components/Chart.svelte';
	import * as Plot from '@observablehq/plot';
	import type { Match } from './utils';
	import Figure from '$lib/components/lines/Figure.svelte';

	export let matches: Array<Match>;

	const players: Record<string, { offensive: number; defensive: number; matches: number }> = {};

	matches.forEach((m) => {
		m.goals?.scorer.forEach((p) => {
			if (players[p]) {
				if (m.teamA[0] === p || m.teamB[0] === p) {
					players[p].offensive += 1;
				} else {
					players[p].defensive += 1;
				}
				players[p].matches += 1;
			} else {
				players[p] = { offensive: 0, defensive: 0, matches: 0 };
			}
		});
	});

	const data = Object.entries(players).map(([player, { offensive, defensive, matches }]) => ({
		player,
		matches,
		offensive,
		defensive,
	}));
</script>

<Figure
	caption="Figure 7. Goals scored from offensive vs defensive position for all players who played at least 100 matches.">
	<Chart
		options={{
			x: { grid: true },
			y: { grid: true },
			aspectRatio: 1,
			marks: [
				Plot.dot(
					data.filter((d) => d.matches > 100),
					{ x: 'offensive', y: 'defensive' }
				),
			],
		}} />
</Figure>
