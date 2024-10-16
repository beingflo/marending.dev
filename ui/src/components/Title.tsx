import type { JSX } from 'solid-js/jsx-runtime';

export type Props = {
	subtitle: string;
	date: string;
	children: JSX.Element;
};

export const Title = ({ children, subtitle, date }: Props) => (
	<>
		<p class="text-xs md:text-sm text-gray-600 dark:text-gray-200">{date}</p>
		<h1 class="text-4xl md:text-6xl mb-4 text-black dark:text-white font-extrabold text-left">
			{children}
		</h1>
		<h2 class="text-lg md:text-xl mb-24 text-left">{subtitle}</h2>
	</>
);
