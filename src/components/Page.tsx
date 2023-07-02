import { JSX } from 'solid-js/jsx-runtime';

export type Props = {
	children: JSX.Element;
};

export const Page = ({ children }: Props) => (
	<div class="p-4 pt-12 md:p-24 max-w-screen-2xl mx-auto">{children}</div>
);
