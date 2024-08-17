import type { JSX } from 'solid-js/jsx-runtime';

export type Props = {
	children: JSX.Element;
};

export const LiD = ({ children }: Props) => <li class="list-disc">{children}</li>;
