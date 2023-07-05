export const SSGResults = () => (
	<>
		<div class="shadow mt-4 border border-black dark:border-white p-4 lg:-mx-4 shadow-[6px_6px_0_#00000020]">
			<table class="w-full">
				<tbody>
					<tr class="border-b border-black dark:border-white">
						<th class="text-left">Page</th>
						<th class="text-right">Requests / second</th>
						<th class="text-right">Avg. latency</th>
						<th class="text-right">99% latency</th>
					</tr>
					<tr>
						<td class="text-left">/</td>
						<td class="text-right">9'589 rps</td>
						<td class="text-right">9 ms</td>
						<td class="text-right">15 ms</td>
					</tr>
					<tr>
						<td class="text-left">/notes/lines</td>
						<td class="text-right">7'888 rps</td>
						<td class="text-right">12 ms</td>
						<td class="text-right">25 ms</td>
					</tr>
				</tbody>
			</table>
		</div>
		<div class="pt-4 pb-6 text-center text-gray-800 dark:text-gray-100 text-sm italic font-serif">
			Table 2: Performance characteristics of a SvelteKit node server with build-time prerendering
			enabled.
		</div>
	</>
);
