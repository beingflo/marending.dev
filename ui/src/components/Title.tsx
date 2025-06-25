import type { JSX } from 'solid-js/jsx-runtime';

export type Props = {
	subtitle: string;
	date: string;
	children: JSX.Element;
};

export const Title = ({ children, subtitle, date }: Props) => (
	<>
		<p class="not-prose text-xs md:text-sm text-gray-600 dark:text-gray-200">{date}</p>
		<h1 class="not-prose text-4xl md:text-6xl mb-4 text-black dark:text-white font-extrabold text-left">
			{children}
		</h1>
		<h2 class="not-prose text-lg md:text-xl mb-24 font-normal text-left">{subtitle}</h2>
	</>
);
