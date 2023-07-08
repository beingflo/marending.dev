import type { JSX } from 'solid-js/jsx-runtime';

export type Props = {
	title: string;
	children?: JSX.Element;
};

export const Details = ({ title, children }: Props) => (
	<details class="mb-3 border border-black dark:border-white lg:-mx-4 shadow-[6px_6px_0_#00000020] dark:shadow-[6px_6px_0_#FFFFFF40]">
		<summary class="cursor-pointer p-2">{title}</summary>
		<div class="p-2 pt-0">{children}</div>
	</details>
);
