import { For } from 'solid-js';
import { Figure } from '../../../components/Figure';

export const ChaosAngles90 = () => {
	const width = 600;
	const height = 400;

	const num = 500;
	const length = 30;

	const getRandomTransformation = () => {
		const rotation = Math.random() * 10 + 85;
		const xTranslate = Math.random() * width;
		const yTranslate = Math.random() * height;
		return {
			transform: `translate(${xTranslate}, ${yTranslate}) rotate(${rotation})`,
		};
	};

	return (
		<Figure caption="Fig 5. More and shorter randomly placed lines with angles between 85 and 95 degrees.">
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
