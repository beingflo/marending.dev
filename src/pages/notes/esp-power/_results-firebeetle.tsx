import { Table } from '../../../components';

export const ResultsFirebeetle = () => (
	<>
		<Table>
			<tr class="border-b border-black dark:border-white font-normal">
				<th class="text-left">Scenario</th>
				<th class="text-right">Firebeetle ESP32</th>
				<th class="text-right">Firebeetle ESP32-E (stock)</th>
				<th class="text-right">Firebeetle ESP32-E (low power)</th>
			</tr>
			<tr>
				<td class="text-left">Wifi setup</td>
				<td class="text-right">60 - 80 mA</td>
				<td class="text-right">70 - 140 mA</td>
				<td class="text-right">70 - 140 mA</td>
			</tr>
			<tr>
				<td class="text-left">Blink in loop</td>
				<td class="text-right">36 mA</td>
				<td class="text-right">41 mA</td>
				<td class="text-right">40.5 mA</td>
			</tr>
			<tr>
				<td class="text-left">Light sleep with LED</td>
				<td class="text-right">1.5 mA</td>
				<td class="text-right">3.2 mA</td>
				<td class="text-right">2.65 mA</td>
			</tr>
			<tr>
				<td class="text-left">Light sleep no LED</td>
				<td class="text-right">1.2 mA</td>
				<td class="text-right">2.6 mA</td>
				<td class="text-right">2.2 mA</td>
			</tr>
			<tr>
				<td class="text-left">Deep sleep</td>
				<td class="text-right">10 uA</td>
				<td class="text-right">470 uA</td>
				<td class="text-right">12 uA</td>
			</tr>
		</Table>
		<div class="pt-4 pb-6 text-center text-gray-800 dark:text-gray-100 text-sm italic font-serif">
			Table 1: Power consumption of ESP32 dev boards in various situations
		</div>
	</>
);
