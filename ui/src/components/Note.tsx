import type { JSX } from 'solid-js/jsx-runtime';

export type Props = {
	children: JSX.Element;
};

export const Note = ({ children }: Props) => (
	<article class="py-8 max-w-4xl mx-auto font-light">{children}</article>
);
