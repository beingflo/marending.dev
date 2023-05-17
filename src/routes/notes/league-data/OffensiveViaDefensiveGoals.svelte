<script lang="ts">
	import Chart from '$lib/components/Chart.svelte';
	import * as Plot from '@observablehq/plot';
	import type { Match } from './utils';
	import Figure from '$lib/components/lines/Figure.svelte';
	import A from '$lib/components/A.svelte';
	import { d } from 'svelte-highlight/languages';

	export let matches: Array<Match>;

	const players: Record<string, { offensive: number; defensive: number; matches: number }> = {};

	matches.forEach((m) => {
		m.goals?.scorer.forEach((p) => {
			if (players[p]) {
				if (m.teamA[0] === p || m.teamB[0] === p) {
					players[p].offensive += 1;
				} else if (m.teamA[1] === p || m.teamB[1] === p) {
					players[p].defensive += 1;
				}
				players[p].matches += 1;
			} else {
				players[p] = { offensive: 0, defensive: 0, matches: 0 };
			}
		});
	});

	let data = Object.entries(players).map(([player, { offensive, defensive, matches }]) => ({
		player,
		matches,
		offensive: offensive / matches,
		defensive: defensive / matches,
	}));

	data = data.filter((d) => d.matches > 10);
</script>

<Figure
	caption="Figure 7. Proportion of goals scored from offensive (circle) vs defensive (dot) position for all players who played at least 10 matches.">
	<Chart
		options={{
			y: { tickFormat: (d) => '' },
			x: { grid: true, label: 'goal rate  →' },
			marks: [
				Plot.link(data, {
					x1: 'defensive',
					x2: 'offensive',
					markerStart: 'circle-stroke',
					markerEnd: 'dot',
					y: 'player',
					sort: { y: 'x1' },
				}),
			],
		}} />
</Figure>
