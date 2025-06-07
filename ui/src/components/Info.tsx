import type { JSX } from 'solid-js/jsx-runtime';

export type Props = {
	children: JSX.Element;
};

export const Info = ({ children }: Props) => (
	<div class="container border border-sky-400 p-2 mb-6 mt-2 shadow-[6px_6px_0_#74d4ff]">
		{children}
	</div>
);
