import { For } from 'solid-js';
import { Figure } from './Figure';

export const ChaosWrong = () => {
	const width = 600;
	const height = 400;

	const num = 200;
	const length = 50;

	const getRandomTransformation = () => {
		const rotation = Math.random() * 45;
		const xTranslate = Math.random() * width;
		const yTranslate = Math.random() * height;
		return {
			transform: `rotate(${rotation}), translate(${xTranslate}, ${yTranslate})`,
		};
	};

	return (
		<Figure>
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
