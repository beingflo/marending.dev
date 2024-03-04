import { Table } from '../../../components';

export const Resultsx86 = () => (
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
					<td class="text-right">73'800/s</td>
					<td class="text-right">14us</td>
				</tr>
				<tr>
					<td class="text-left">Vanilla (write)</td>
					<td class="text-right">925/s</td>
					<td class="text-right">1'484us</td>
				</tr>
				<tr>
					<td class="text-left">WAL Mode (read)</td>
					<td class="text-right">332'225/s</td>
					<td class="text-right">4us</td>
				</tr>
				<tr>
					<td class="text-left">WAL Mode (write)</td>
					<td class="text-right">5'542/s</td>
					<td class="text-right">211us</td>
				</tr>
				<tr>
					<td class="text-left">WAL + Synchronisation Normal (read)</td>
					<td class="text-right">315'756/s</td>
					<td class="text-right">4us</td>
				</tr>
				<tr>
					<td class="text-left">WAL + Synchronisation Normal (write)</td>
					<td class="text-right">80'145/s</td>
					<td class="text-right">12us</td>
				</tr>
				<tr>
					<td class="text-left">WAL + In-memory (read)</td>
					<td class="text-right">952'380/s</td>
					<td class="text-right">1us</td>
				</tr>
				<tr>
					<td class="text-left">WAL + In-memory (write)</td>
					<td class="text-right">322'695/s</td>
					<td class="text-right">3us</td>
				</tr>
				<tr>
					<td class="text-left">WAL + Index (read)</td>
					<td class="text-right">315'357/s</td>
					<td class="text-right">4us</td>
				</tr>
				<tr>
					<td class="text-left">WAL + Index (write)</td>
					<td class="text-right">26'469/s</td>
					<td class="text-right">34us</td>
				</tr>
				<tr>
					<td class="text-left">WAL + Index (mixed, 80% read)</td>
					<td class="text-right">121'703/s</td>
					<td class="text-right">15us</td>
				</tr>
				<tr>
					<td class="text-left">WAL + Index (mixed 80% write)</td>
					<td class="text-right">36'260/s</td>
					<td class="text-right">26us</td>
				</tr>
			</Table>
		</div>
		<div class="pt-4 pb-6 text-center text-gray-800 dark:text-gray-100 text-sm italic font-serif">
			Table 3: Linux AMD x86 results (Hetzner CPX31).
		</div>
	</>
);
