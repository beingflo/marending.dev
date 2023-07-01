<div class="md:w-2/3 mx-auto shadow mt-4 border border-black dark:border-white p-4">
	<table class="w-full">
		<tr class="border-b border-black dark:border-white">
			<th align="left">Mode</th>
			<th align="right">FireBeetle ESP32</th>
			<th align="right">Seeed XIAO</th>
		</tr>
		<tr>
			<td align="left">Deep sleep</td>
			<td align="right">4.5 s</td>
			<td align="right">1.3 s</td>
		</tr>
		<tr>
			<td align="left">Deep sleep + static IP</td>
			<td align="right">4.4 s</td>
			<td align="right">1.4 s</td>
		</tr>
	</table>
</div>
<div class="pt-4 pb-6 text-center text-gray-800 dark:text-gray-100 text-sm italic font-serif">
	Table 1: Time from wake up to successful transmission of small message with WiFi
</div>

<style>
	th {
		font-weight: normal;
	}
	.shadow {
		box-shadow: 6px 6px 0 #00000020;
	}
</style>
