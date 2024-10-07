import type { JSX } from 'solid-js/jsx-runtime';

export type Props = {
	children: JSX.Element;
};

export const WidePage = ({ children }: Props) => (
	<div class="p-4 md:p-24 md:pt-32 pt-16 max-w-screen-4xl mx-auto">{children}</div>
);
