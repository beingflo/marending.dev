import { JSX } from 'solid-js/jsx-runtime';

export type Props = {
	pb?: number;
	children: JSX.Element;
};

export const P = ({ pb = 4, children }: Props) => (
	<p style={`padding-bottom: ${pb / 4}rem`}>{children}</p>
);
