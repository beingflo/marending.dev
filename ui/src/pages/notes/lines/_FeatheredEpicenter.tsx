import { For } from 'solid-js';
import { Figure } from 'src/components/Figure';

export const FeatheredEpicenter = () => {
	const width = 600;
	const height = 400;

	const num = 500;
	const length = 30;

	// Point of disturbance
	const PoD = [400, 150];

	const maxDistance = 100 * 100;

	const getRandomTransformation = () => {
		const translate = [Math.random() * width, Math.random() * height];
		const dX = Math.abs(PoD[0] - translate[0]);
		const dY = Math.abs(PoD[1] - translate[1]);

		const distance2 = dX * dX + dY * dY;

		let rotation = Math.max((maxDistance - distance2) / maxDistance, 0) * 90;

		return {
			transform: `translate(${translate[0]}, ${translate[1]}) rotate(${rotation})`,
		};
	};

	return (
		<Figure caption="Fig 7. Lines turn more upright the closer they get to the epicenter. Circle represents epicenter.">
			<div class="w-full sm:w-[600px]">
				<svg
					viewBox={`0 0 ${width} ${height}`}
					xmlns="http://www.w3.org/2000/svg">
					<For each={Array(num)}>
						{() => (
							<line
								{...getRandomTransformation()}
								x1={-length / 2}
								y1="0"
								x2={length / 2}
								y2="0"
								stroke="currentColor"
							/>
						)}
					</For>
				</svg>
			</div>
		</Figure>
	);
};
