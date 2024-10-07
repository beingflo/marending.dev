import { Show } from 'solid-js';
import type { JSX } from 'solid-js/jsx-runtime';

export type Props = {
	caption?: string;
	children: JSX.Element;
};

export const Figure = ({ caption, children }: Props) => (
	<div class="w-full sm:w-fit mx-auto pb-6 pt-4">
		{children}
		<Show when={caption}>
			<div class="pt-4 text-center text-gray-800 dark:text-gray-100 text-sm italic font-serif">
				{caption}
			</div>
		</Show>
	</div>
);
