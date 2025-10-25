import { Show, type JSX } from 'solid-js';
import { A } from './A';

export type Props = {
	name: string;
	link?: string;
	children: JSX.Element;
	technologies: string;
	note?: string;
};

export const ProjectEntry = ({ name, link, children, technologies, note }: Props) => (
	<>
		<div class="flex flex-col txt-right">
			<Show
				when={!!link}
				fallback={<div>{name}</div>}>
				<A href={link!}>{name}</A>
			</Show>
			<Show when={!!note}>
				<div class="text-xs font-thin">
					<A href={note!}>{note}</A>
				</div>
			</Show>
		</div>
		<div class="col-span-2 mb-8 md:mb-0 flex flex-col">
			<span>{children}</span>
			<div class="mt-2 text-sm text-stone-600">
				<span>Technologies involved: </span>
				<span>{technologies}</span>
			</div>
		</div>
	</>
);
