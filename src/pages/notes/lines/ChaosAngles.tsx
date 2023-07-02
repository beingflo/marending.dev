import { For } from 'solid-js';
import { Figure } from './Figure';

export const ChaosAngles = () => {
	const width = 600;
	const height = 400;

	const num = 200;
	const length = 50;

	const getRandomTransformation = () => {
		const rotation = Math.random() * 10 - 5;
		const xTranslate = Math.random() * width;
		const yTranslate = Math.random() * height;
		return {
			transform: `translate(${xTranslate}, ${yTranslate}) rotate(${rotation})`,
		};
	};
	return (
		<Figure caption="Fig 4. Randomly placed lines with angles between -5 and 5 degrees.">
			<div class="w-full sm:w-[600px]">
				<svg viewBox={`0 0 ${width} ${height}`} xmlns="http://www.w3.org/2000/svg">
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
