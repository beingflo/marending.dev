import type { JSX } from 'solid-js/jsx-runtime';

export type Props = {
	children: JSX.Element;
};

export const Note = ({ children }: Props) => (
	<article class="p-4 py-8 md:py-12 max-w-4xl mx-auto text-justify">{children}</article>
);
