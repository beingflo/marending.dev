<script>
	import Figure from '$lib/components/lines/Figure.svelte';

	export const width = 600;
	export const height = 400;

	const num = 200;
	const length = 50;

	const getRandomTransformation = () => {
		const rotation = Math.random() * 180;
		const xTranslate = Math.random() * width;
		const yTranslate = Math.random() * height;
		return {
			transform: `translate(${xTranslate}, ${yTranslate}) rotate(${rotation})`,
		};
	};
</script>

<Figure caption="Fig 3. {num} randomly translated and rotated lines">
	<div style="width: {width}px; height: {height}px;">
		<svg width="100%" height="100%" xmlns="http://www.w3.org/2000/svg">
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
