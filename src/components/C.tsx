import type { JSX } from 'solid-js/jsx-runtime';

export type Props = {
	children: JSX.Element;
};

export const C = ({ children }: Props) => (
	<code class="font-mono bg-gray-200 dark:bg-gray-700 p-0.5 rounded-sm">{children}</code>
);
