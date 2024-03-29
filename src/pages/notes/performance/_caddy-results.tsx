import { Table } from '../../../components';

export const CaddyResults = () => (
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
				<td class="text-right">6'041 rps</td>
				<td class="text-right">16 ms</td>
				<td class="text-right">25 ms</td>
			</tr>
			<tr>
				<td class="text-left">/notes/lines</td>
				<td class="text-right">4'342 rps</td>
				<td class="text-right">22 ms</td>
				<td class="text-right">31 ms</td>
			</tr>
		</Table>
		<div class="pt-4 pb-6 text-center text-gray-800 dark:text-gray-100 text-sm italic font-serif">
			Table 3: Performance characteristics of a SvelteKit node server serving static assets.
			Requests are reverse proxied via Caddy.
		</div>
	</>
);
