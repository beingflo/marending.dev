import type { JSX } from 'solid-js/jsx-runtime';

export type Props = {
	children: JSX.Element;
};

export const Li = ({ children }: Props) => <li class="list-decimal">{children}</li>;
