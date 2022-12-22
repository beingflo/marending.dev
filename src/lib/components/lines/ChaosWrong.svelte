<script>
	import { browser } from '$app/environment';
	import Figure from '$lib/components/lines/Figure.svelte';

	export const width = 600;
	export const height = 400;

	const num = browser ? 200 : 0;
	const length = 50;

	const getRandomTransformation = () => {
		const rotation = Math.random() * 45;
		const xTranslate = Math.random() * width;
		const yTranslate = Math.random() * height;
		return {
			transform: `rotate(${rotation}), translate(${xTranslate}, ${yTranslate})`,
		};
	};
</script>

<Figure>
	<div class="w-full sm:w-[600px]">
		<svg viewBox="0 0 {width} {height}" xmlns="http://www.w3.org/2000/svg">
			{#each Array(num) as _}
				<line
					{...getRandomTransformation()}
					x1={-length / 2}
					y1="0"
					x2={length / 2}
					y2="0"
					stroke="currentColor" />
			{/each}
		</svg>
	</div>
</Figure>
