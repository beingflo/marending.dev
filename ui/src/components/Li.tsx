import type { JSX } from 'solid-js/jsx-runtime';

export type Props = {
	children: JSX.Element;
};

export const Li = ({ children }: Props) => <li class="list-decimal mx-4 lg:mx-0">{children}</li>;
