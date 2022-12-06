<script>
	import Figure from '$lib/components/lines/Figure.svelte';

	const width = 600;
	const height = 400;

	const num = 500;
	const length = 30;

	// Point of disturbance
	const PoD = [Math.random() * width, Math.random() * height];
	const distance = 100;

	const getRandomTransformation = () => {
		const translate = [Math.random() * width, Math.random() * height];
		let rotation = 0;
		const dX = Math.abs(PoD[0] - translate[0]);
		const dY = Math.abs(PoD[1] - translate[1]);
		if (dX * dX + dY * dY < distance * distance) {
			rotation = Math.random() * 20 + 80;
		}
		return {
			transform: `translate(${translate[0]}, ${translate[1]}) rotate(${rotation})`,
		};
	};
</script>

<Figure caption="Fig 6. Horizontal lines with vertical region around random point">
	<div class="w-full sm:w-[600px]">
		<svg width="100%" {height} xmlns="http://www.w3.org/2000/svg">
			{#each Array(num) as _}
				<line
					{...getRandomTransformation()}
					x1={-length / 2}
					y1="0"
					x2={length / 2}
					y2="0"
					stroke="black" />
			{/each}
		</svg>
	</div>
</Figure>
