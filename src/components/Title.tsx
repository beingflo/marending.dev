import { JSX } from 'solid-js/jsx-runtime';

export type Props = {
	subtitle: string;
	children: JSX.Element;
};

export const Title = ({ children, subtitle }: Props) => (
	<>
		<h1 class="text-4xl md:text-6xl md:mb-2 text-black dark:text-white font-extrabold text-left">
			{children}
		</h1>
		<h2 class="text-lg md:text-xl mb-16 md:mb-24 text-left">{subtitle}</h2>
	</>
);