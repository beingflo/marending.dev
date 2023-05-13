<script lang="ts">
	import Chart from '$lib/components/Chart.svelte';
	import * as Plot from '@observablehq/plot';
	import type { Match } from './utils';
	import Figure from '$lib/components/lines/Figure.svelte';

	export let matches: Array<Match>;

	const matchesPerPlayer: Record<string, number> = {};

	matches.forEach((m) => {
		m.players.forEach((p) => {
			if (matchesPerPlayer[p]) {
				matchesPerPlayer[p] += 1;
			} else {
				matchesPerPlayer[p] = 1;
			}
		});
	});

	const data = Object.entries(matchesPerPlayer).map(([p, n]) => ({ player: p, matches: n }));
	data.sort((a, b) => b.matches - a.matches);
</script>

<Figure caption="Figure 3. Number of matches by player. Limited to the 100 top players">
	<Chart
		options={{
			x: { tickSize: 0, tickFormat: (d) => '', label: 'Players' },
			y: { grid: true },
			marks: [
				Plot.barY(data.slice(0, 100), {
					x: 'player',
					y: 'matches',
					sort: { x: 'y', reverse: true },
				}),
				Plot.text(data.slice(0, 100), {
					x: 'player',
					y: 'matches',
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
