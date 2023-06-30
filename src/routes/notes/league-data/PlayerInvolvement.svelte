<script lang="ts">
	import Chart from '$lib/components/Chart.svelte';
	import * as Plot from '@observablehq/plot';
	import type { Match } from './utils';
	import Figure from '$lib/components/lines/Figure.svelte';

	export let matches: Array<Match>;

	const involvement: Record<string, number> = {};
	const players: Record<string, number> = {};

	matches.forEach((m) => {
		m.players.forEach((p) => {
			if (players[p] === undefined) {
				players[p] = 0;
			}
			players[p] += 1;
		});
	});

	const playersLimited = Object.entries(players).filter((p) => p[1] > 100);
	console.log(playersLimited);

	matches.forEach((m) => {
		const players = m.players;
		players.forEach((p1) => {
			players.forEach((p2) => {
				if (p1 !== p2) {
					const key = [p1, p2].sort();
					if (involvement[`${key[0]}-${key[1]}`] === undefined) {
						involvement[`${key[0]}-${key[1]}`] = 0;
					}
					involvement[`${key[0]}-${key[1]}`] += 1;
				}
			});
		});
	});
	console.log(Object.entries(involvement).sort((a, b) => a[1] - b[1]));
</script>

<Figure caption="Figure 2. Moving average of the duration of games">
	<Chart
		options={{
			marks: [
				Plot.cell([...Array(playersLimited.length).keys()], {
					x: (d) => playersLimited[d],
					y: (d) => playersLimited[d],
				}),
			],
		}} />
</Figure>
