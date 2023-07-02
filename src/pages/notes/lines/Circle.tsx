import { Figure } from './Figure';

export const Circle = () => (
	<Figure caption="Fig 1. Not a straight line">
		<div class="w-fit mx-auto">
			<svg width="160" viewBox="0 0 160 160" xmlns="http://www.w3.org/2000/svg">
				<circle cx="80" cy="80" r="50" fill-opacity="0" stroke="currentColor" />
			</svg>
		</div>
	</Figure>
);
