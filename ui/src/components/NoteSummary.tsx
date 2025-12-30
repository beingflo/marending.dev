export type Props = {
	title: string;
	summary: string;
	date: string;
	link: string;
};

export const NoteSummary = ({ title, summary, date, link }: Props) => (
	<a
		class="w-full"
		href={link}>
		<div class="flex flex-row justify-between w-full">
			<div class="flex flex-wrap gap-x-2 items-baseline">
				<h3 class="font-semibold">{title}</h3>
				<p class="text-sm font-light">{summary}</p>
			</div>
			<p class="text-sm font-light hidden lg:block">{date}</p>
		</div>
	</a>
);
