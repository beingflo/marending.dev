import { Table } from '../../../components';

export const SSRResults = () => (
	<>
		<Table>
			<tr class="border-b border-black dark:border-white">
				<th class="text-left">Page</th>
				<th class="text-right">Requests / second</th>
				<th class="text-right">Avg. latency</th>
				<th class="text-right">99% latency</th>
			</tr>
			<tr>
				<td class="text-left">/</td>
				<td class="text-right">2'095 rps</td>
				<td class="text-right">44 ms</td>
				<td class="text-right">88 ms</td>
			</tr>
			<tr>
				<td class="text-left">/notes/lines</td>
				<td class="text-right">455 rps</td>
				<td class="text-right">207 ms</td>
				<td class="text-right">268 ms</td>
			</tr>
		</Table>
		<div class="pt-4 pb-6 text-center text-gray-800 dark:text-gray-100 text-sm italic font-serif">
			Table 1: Performance characteristics of a SvelteKit node server with server-side rendering
			enabled.
		</div>
	</>
);
