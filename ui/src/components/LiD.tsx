import type { JSX } from 'solid-js/jsx-runtime';

export type Props = {
	children: JSX.Element;
};

export const LiD = ({ children }: Props) => <li class="list-disc mx-4 lg:mx-0">{children}</li>;
