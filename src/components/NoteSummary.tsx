export type Props = {
	title: string;
	summary: string;
	date: string;
	link: string;
};

export const NoteSummary = ({ title, summary, date, link }: Props) => (
	<a class="w-fit" href={link}>
		<p class="text-sm text-gray-600 dark:text-gray-200">{date}</p>
		<h3 class="font-semibold">{title}</h3>
		<p class="text-sm font-light">{summary}</p>
	</a>
);
