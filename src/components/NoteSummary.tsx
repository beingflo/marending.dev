export type Props = {
	title: string;
	summary: string;
	date: string;
	link: string;
};

export const NoteSummary = ({ title, summary, date, link }: Props) => (
	<a
		class="flex flex-row hover:text-stone-600 dark:hover:text-stone-400"
		href={link}>
		<p class="text-sm w-16 md:w-1/6">{date}</p>
		<div class="flex flex-col xl:flex-row items-baseline md:gap-2 w-5/6">
			<h3 class="font-semibold w-fit">{title}</h3>
			<p class="text-sm font-light">{summary}</p>
		</div>
	</a>
);
