import { JSX } from 'solid-js/jsx-runtime';

export type Props = {
	id?: string;
	children: JSX.Element;
};

export const H3 = ({ id, children }: Props) => (
	<h3 id={id} class="text-lg font-semibold mb-2 mt-2">
		{children}
	</h3>
);
