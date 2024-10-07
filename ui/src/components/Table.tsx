import type { JSX } from 'solid-js/jsx-runtime';

export type Props = {
	children?: JSX.Element;
};

export const Table = ({ children }: Props) => (
	<div class="mt-4 border border-black dark:border-white p-4 lg:-mx-4 shadow-[6px_6px_0_#00000020] dark:dark:shadow-[6px_6px_0_#FFFFFF40]">
		<table class="w-full">
			<tbody>{children}</tbody>
		</table>
	</div>
);
