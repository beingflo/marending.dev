import { Table } from '../../../components';

export const CaddyStaticResults = () => (
	<>
		<Table>
			<tr class="border-b border-black dark:boder-white">
				<th class="text-left">Page</th>
				<th class="text-right">Requests / second</th>
				<th class="text-right">Avg. latency</th>
				<th class="text-right">99% latency</th>
			</tr>
			<tr>
				<td class="text-left">/</td>
				<td class="text-right">46'553 rps</td>
				<td class="text-right">1.6 ms</td>
				<td class="text-right">24.6 ms</td>
			</tr>
			<tr>
				<td class="text-left">/notes/lines</td>
				<td class="text-right">42'973 rps</td>
				<td class="text-right">2.4 ms</td>
				<td class="text-right">43.8 ms</td>
			</tr>
		</Table>
		<div class="pt-4 pb-6 text-center text-gray-800 dark:text-gray-100 text-sm italic font-serif">
			Table 4: Performance characteristics of Caddy serving static pages built by SvelteKits static
			adapter.
		</div>
	</>
);
