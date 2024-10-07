import type { JSX } from 'solid-js/jsx-runtime';

export type Props = {
	id?: string;
	children: JSX.Element;
};

export const H2 = ({ id, children }: Props) => (
	<h2
		id={id}
		class="text-xl font-semibold mb-3 mt-6">
		{children}
	</h2>
);
