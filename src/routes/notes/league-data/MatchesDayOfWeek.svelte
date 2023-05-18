<script lang="ts">
	import Chart from '$lib/components/Chart.svelte';
	import * as Plot from '@observablehq/plot';
	import type { Match } from './utils';
	import Figure from '$lib/components/lines/Figure.svelte';

	export let matches: Array<Match>;
</script>

<Figure caption="Figure 3. Day of week distribution of all games">
	<Chart
		options={{
			y: { grid: true, label: '↑ matches' },
			x: { label: 'day', tickFormat: Plot.formatWeekday('en-US', 'short') },
			marks: [Plot.barY(matches, Plot.groupX({ y: 'count' }, { x: (d) => d.startDate.getDay() }))],
		}} />
</Figure>
