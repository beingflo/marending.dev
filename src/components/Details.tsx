export const Details = ({ title, children }: { title: string; children: any }) => (
	<details class="mb-3 border border-black dark:border-white lg:-mx-4 shadow-[6px_6px_0_#00000020]">
		<summary class="cursor-pointer p-2">{title}</summary>
		<div class="p-2 pt-0">{children}</div>
	</details>
);
