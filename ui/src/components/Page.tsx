import type { JSX } from 'solid-js/jsx-runtime';

export type Props = {
	children: JSX.Element;
};

export const Page = ({ children }: Props) => <div class="pt-12 md:py-24">{children}</div>;
