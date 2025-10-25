export type Props = {
	title: string;
	summary: string;
	date: string;
	link: string;
};

export const NoteSummary = ({ title, summary, date, link }: Props) => (
	<a
		class="flex flex-row items-baseline md:gap-8 w-full md:w-3/4 max-w-4xl mx-auto md:hover:text-stone-500 md:dark:hover:text-stone-400"
		href={link}>
		<p class="text-sm w-16 md:w-24 font-light">{date}</p>
		<div class="flex flex-col w-full items-baseline">
			<h3 class="font-semibold w-full underline">{title}</h3>
			<p class="text-sm font-light">{summary}</p>
		</div>
	</a>
);
