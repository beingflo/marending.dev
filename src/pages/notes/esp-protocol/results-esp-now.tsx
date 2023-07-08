import { Table } from '../../../components';

export const ResultsEspNow = () => (
	<>
		<div class="md:w-1/2 mx-auto">
			<Table>
				<tr class="border-b border-black dark:border-white">
					<th class="text-left">Sleep mode</th>
					<th class="text-right">FireBeetle ESP32</th>
					<th class="text-right">Seeed XIAO</th>
				</tr>
				<tr>
					<td class="text-left">Deep sleep</td>
					<td class="text-right">416 ms</td>
					<td class="text-right">172 ms</td>
				</tr>
				<tr>
					<td class="text-left">Light sleep</td>
					<td class="text-right">104 ms</td>
					<td class="text-right">102 ms</td>
				</tr>
			</Table>
		</div>
		<div class="pt-4 pb-6 text-center text-gray-800 dark:text-gray-100 text-sm italic font-serif">
			Table 2: Time from wake up to successful transmission of small message with ESP-NOW
		</div>
	</>
);
