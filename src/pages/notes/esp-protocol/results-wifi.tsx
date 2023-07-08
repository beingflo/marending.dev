import { Table } from '../../../components';

export const ResultsWifi = () => (
	<>
		<div class="md:w-2/3 mx-auto">
			<Table>
				<tr class="border-b border-black dark:border-white">
					<th class="text-left">Mode</th>
					<th class="text-right">FireBeetle ESP32</th>
					<th class="text-right">Seeed XIAO</th>
				</tr>
				<tr>
					<td class="text-left">Deep sleep</td>
					<td class="text-right">4.5 s</td>
					<td class="text-right">1.3 s</td>
				</tr>
				<tr>
					<td class="text-left">Deep sleep + static IP</td>
					<td class="text-right">4.4 s</td>
					<td class="text-right">1.4 s</td>
				</tr>
			</Table>
		</div>
		<div class="pt-4 pb-6 text-center text-gray-800 dark:text-gray-100 text-sm italic font-serif">
			Table 1: Time from wake up to successful transmission of small message with WiFi
		</div>
	</>
);
