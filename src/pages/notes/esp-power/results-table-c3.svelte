<div class="shadow mt-4 border border-black dark:border-white p-4 lg:-mx-4">
	<table class="w-full">
		<tr class="border-b border-black dark:border-white">
			<th align="left">Scenario</th>
			<th align="right">Seeed XIAO</th>
			<th align="right">M5Stamp</th>
		</tr>
		<tr>
			<td align="left">Wifi setup</td>
			<td align="right">30 - 60 mA</td>
			<td align="right">30 - 60 mA</td>
		</tr>
		<tr>
			<td align="left">Blink in loop</td>
			<td align="right">18 mA</td>
			<td align="right">18 mA</td>
		</tr>
		<tr>
			<td align="left">Light sleep with LED</td>
			<td align="right">1.5 mA</td>
			<td align="right">2.2 mA</td>
		</tr>
		<tr>
			<td align="left">Light sleep no LED</td>
			<td align="right">0.2 mA</td>
			<td align="right">0.95 mA</td>
		</tr>
		<tr>
			<td align="left">Deep sleep</td>
			<td align="right">38 uA</td>
			<td align="right">750 uA</td>
		</tr>
	</table>
</div>
<div class="pt-4 pb-6 text-center text-gray-800 dark:text-gray-100 text-sm italic font-serif">
	Table 2: Power consumption of ESP32-C3 dev boards in various situations
</div>

<style>
	th {
		font-weight: normal;
	}
	.shadow {
		box-shadow: 6px 6px 0 #00000020;
	}
</style>
