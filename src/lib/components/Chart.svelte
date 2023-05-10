<script lang="ts">
	import * as Plot from '@observablehq/plot';

	export let options: Plot.PlotOptions;

	function myplot(node: any) {
		node.appendChild(Plot.plot(options));
	}
</script>

{#key options}
	<div class="flex justify-center" use:myplot {...$$restProps} />
{/key}
