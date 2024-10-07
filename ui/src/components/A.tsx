import type { JSX } from 'solid-js/jsx-runtime';

export type Props = {
	href: string;
	children: JSX.Element;
};

export const A = ({ href, children }: Props) => (
	<a
		href={href}
		class="underline decoration-1 border-dashed">
		{children}
	</a>
);
