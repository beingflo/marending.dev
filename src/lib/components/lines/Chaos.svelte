<script>
	import Figure from '$lib/components/lines/Figure.svelte';

	export const width = 600;
	export const height = 400;

	const num = 100;
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

<Figure>
	<svg {width} {height} xmlns="http://www.w3.org/2000/svg">
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
</Figure>
