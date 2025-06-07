import type { JSX } from 'solid-js/jsx-runtime';

export type Props = {
	children: JSX.Element;
};

export const Hint = ({ children }: Props) => (
	<div class="container border border-lime-700 p-2 mb-6 mt-2 shadow-[6px_6px_0_rgba(77,124,15,0.2)]">
		{children}
	</div>
);
