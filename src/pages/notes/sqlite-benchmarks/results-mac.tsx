import { Table } from '../../../components';

export const ResultsMac = () => (
	<>
		<div class="md:w-full mx-auto">
			<Table>
				<tr class="border-b border-black dark:border-white">
					<th class="text-left">Scenario</th>
					<th class="text-right">Throughput</th>
					<th class="text-right">90 percentile duration</th>
				</tr>
				<tr>
					<td class="text-left">Vanilla (read)</td>
					<td class="text-right">183'150/s</td>
					<td class="text-right">6us</td>
				</tr>
				<tr>
					<td class="text-left">Vanilla (write)</td>
					<td class="text-right">4'363/s</td>
					<td class="text-right">248us</td>
				</tr>
				<tr>
					<td class="text-left">WAL Mode (read)</td>
					<td class="text-right">454'338/s</td>
					<td class="text-right">3us</td>
				</tr>
				<tr>
					<td class="text-left">WAL Mode (write)</td>
					<td class="text-right">14'401/s</td>
					<td class="text-right">37us</td>
				</tr>
				<tr>
					<td class="text-left">WAL + Synchronisation Normal (read)</td>
					<td class="text-right">483'558/s</td>
					<td class="text-right">3us</td>
				</tr>
				<tr>
					<td class="text-left">WAL + Synchronisation Normal (write)</td>
					<td class="text-right">113'684/s</td>
					<td class="text-right">8us</td>
				</tr>
				<tr>
					<td class="text-left">WAL + In-memory (read)</td>
					<td class="text-right">50'000'000/s</td>
					<td class="text-right">0us</td>
				</tr>
				<tr>
					<td class="text-left">WAL + In-memory (write)</td>
					<td class="text-right">981'836/s</td>
					<td class="text-right">1us</td>
				</tr>
				<tr>
					<td class="text-left">WAL + Index (read)</td>
					<td class="text-right">440'917/s</td>
					<td class="text-right">3us</td>
				</tr>
				<tr>
					<td class="text-left">WAL + Index (write)</td>
					<td class="text-right">47'359/s</td>
					<td class="text-right">25us</td>
				</tr>
				<tr>
					<td class="text-left">WAL + Index (mixed, 80% read)</td>
					<td class="text-right">197'012/s</td>
					<td class="text-right">15us</td>
				</tr>
				<tr>
					<td class="text-left">WAL + Index (mixed 80% write)</td>
					<td class="text-right">59'111/s</td>
					<td class="text-right">16us</td>
				</tr>
			</Table>
		</div>
		<div class="pt-4 pb-6 text-center text-gray-800 dark:text-gray-100 text-sm italic font-serif">
			Table 1: M1 Macbook Air results.
		</div>
	</>
);
