import { Table } from '../../../components';

export const ResultsArm = () => (
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
					<td class="text-right">94'786/s</td>
					<td class="text-right">12us</td>
				</tr>
				<tr>
					<td class="text-left">Vanilla (write)</td>
					<td class="text-right">560/s</td>
					<td class="text-right">2'463us</td>
				</tr>
				<tr>
					<td class="text-left">WAL Mode (read)</td>
					<td class="text-right">238'322/s</td>
					<td class="text-right">6us</td>
				</tr>
				<tr>
					<td class="text-left">WAL Mode (write)</td>
					<td class="text-right">3'316/s</td>
					<td class="text-right">585us</td>
				</tr>
				<tr>
					<td class="text-left">WAL + Synchronisation Normal (read)</td>
					<td class="text-right">275'406/s</td>
					<td class="text-right">5us</td>
				</tr>
				<tr>
					<td class="text-left">WAL + Synchronisation Normal (write)</td>
					<td class="text-right">46'512/s</td>
					<td class="text-right">18us</td>
				</tr>
				<tr>
					<td class="text-left">WAL + In-memory (read)</td>
					<td class="text-right">943'396/s</td>
					<td class="text-right">1us</td>
				</tr>
				<tr>
					<td class="text-left">WAL + In-memory (write)</td>
					<td class="text-right">299'976/s</td>
					<td class="text-right">4us</td>
				</tr>
				<tr>
					<td class="text-left">WAL + Index (read)</td>
					<td class="text-right">186'358/s</td>
					<td class="text-right">7us</td>
				</tr>
				<tr>
					<td class="text-left">WAL + Index (write)</td>
					<td class="text-right">19'479/s</td>
					<td class="text-right">46us</td>
				</tr>
				<tr>
					<td class="text-left">WAL + Index (mixed, 80% read)</td>
					<td class="text-right">75'153/s</td>
					<td class="text-right">24us</td>
				</tr>
				<tr>
					<td class="text-left">WAL + Index (mixed 80% write)</td>
					<td class="text-right">22'801/s</td>
					<td class="text-right">42us</td>
				</tr>
			</Table>
		</div>
		<div class="pt-4 pb-6 text-center text-gray-800 dark:text-gray-100 text-sm italic font-serif">
			Table 2: Linux Ampere ARM results (Hetzner CAX31).
		</div>
	</>
);
