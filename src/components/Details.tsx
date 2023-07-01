export const Details = ({
  title,
  children,
}: {
  title: string;
  children: any;
}) => (
  <details class="mb-3 border border-black dark:border-white lg:-mx-4">
    <summary class="cursor-pointer p-2">{title}</summary>
    <div class="p-2 pt-0">{children}</div>
  </details>
);

{
  /* <style>
	details {
		box-shadow: 6px 6px 0 #00000020;
	}
</style> */
}
