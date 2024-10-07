import { Table } from '../../../components';

export const ResultsC3 = () => (
	<>
		<Table>
			<tr class="border-b border-black dark:border-white">
				<th class="text-left">Scenario</th>
				<th class="text-right">Seeed XIAO</th>
				<th class="text-right">M5Stamp</th>
			</tr>
			<tr>
				<td class="text-left">Wifi setup</td>
				<td class="text-right">30 - 60 mA</td>
				<td class="text-right">30 - 60 mA</td>
			</tr>
			<tr>
				<td class="text-left">Blink in loop</td>
				<td class="text-right">18 mA</td>
				<td class="text-right">18 mA</td>
			</tr>
			<tr>
				<td class="text-left">Light sleep with LED</td>
				<td class="text-right">1.5 mA</td>
				<td class="text-right">2.2 mA</td>
			</tr>
			<tr>
				<td class="text-left">Light sleep no LED</td>
				<td class="text-right">0.2 mA</td>
				<td class="text-right">0.95 mA</td>
			</tr>
			<tr>
				<td class="text-left">Deep sleep</td>
				<td class="text-right">38 uA</td>
				<td class="text-right">750 uA</td>
			</tr>
		</Table>
		<div class="pt-4 pb-6 text-center text-gray-800 dark:text-gray-100 text-sm italic font-serif">
			Table 2: Power consumption of ESP32-C3 dev boards in various situations
		</div>
	</>
);
