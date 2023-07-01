<div class="shadow mt-4 border border-black dark:border-white p-4 lg:-mx-4">
	<table class="w-full">
		<tr class="border-b border-black dark:border-white">
			<th align="left">Scenario</th>
			<th align="right">Firebeetle ESP32</th>
			<th align="right">Firebeetle ESP32-E (stock)</th>
			<th align="right">Firebeetle ESP32-E (low power)</th>
		</tr>
		<tr>
			<td align="left">Wifi setup</td>
			<td align="right">60 - 80 mA</td>
			<td align="right">70 - 140 mA</td>
			<td align="right">70 - 140 mA</td>
		</tr>
		<tr>
			<td align="left">Blink in loop</td>
			<td align="right">36 mA</td>
			<td align="right">41 mA</td>
			<td align="right">40.5 mA</td>
		</tr>
		<tr>
			<td align="left">Light sleep with LED</td>
			<td align="right">1.5 mA</td>
			<td align="right">3.2 mA</td>
			<td align="right">2.65 mA</td>
		</tr>
		<tr>
			<td align="left">Light sleep no LED</td>
			<td align="right">1.2 mA</td>
			<td align="right">2.6 mA</td>
			<td align="right">2.2 mA</td>
		</tr>
		<tr>
			<td align="left">Deep sleep</td>
			<td align="right">10 uA</td>
			<td align="right">470 uA</td>
			<td align="right">12 uA</td>
		</tr>
	</table>
</div>
<div class="pt-4 pb-6 text-center text-gray-800 dark:text-gray-100 text-sm italic font-serif">
	Table 1: Power consumption of ESP32 dev boards in various situations
</div>

<style>
	th {
		font-weight: normal;
	}
	.shadow {
		box-shadow: 6px 6px 0 #00000020;
	}
</style>
