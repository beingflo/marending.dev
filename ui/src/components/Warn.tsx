import type { JSX } from 'solid-js/jsx-runtime';

export type Props = {
	children: JSX.Element;
};

export const Warn = ({ children }: Props) => (
	<div class="container border border-orange-300 p-2 mb-6 mt-2 shadow-[6px_6px_0_rgba(253,186,116,0.3)]">
		{children}
	</div>
);
