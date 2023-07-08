import { Show } from 'solid-js';

export type Props = {
	value: string;
	caption?: string;
	respectMargin?: boolean;
};

export const Code = ({ value, caption = '', respectMargin = false }: Props) => (
	<div class="mb-6">
		<div
			class={`shadow border border-black dark:border-white ${
				respectMargin ? '' : 'lg:-mx-4'
			} shadow-[6px_6px_0_#00000020] dark:shadow-[6px_6px_0_#FFFFFF40]`}>
			<div class="whitespace-pre p-2 font-mono overflow-x-scroll">{value}</div>
		</div>
		<Show when={caption}>
			<div class="pt-4 text-center text-gray-800 dark:text-gray-100 text-sm italic font-serif">
				{caption}
			</div>
		</Show>
	</div>
);
