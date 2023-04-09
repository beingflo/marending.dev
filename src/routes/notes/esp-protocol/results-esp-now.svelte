<div class="md:w-1/2 mx-auto shadow mt-4 border border-black dark:border-white p-4">
	<table class="w-full">
		<tr class="border-b border-black dark:border-white">
			<th align="left">Sleep mode</th>
			<th align="right">FireBeetle ESP32</th>
			<th align="right">Seeed XIAO</th>
		</tr>
		<tr>
			<td align="left">Deep sleep</td>
			<td align="right">416 ms</td>
			<td align="right">172 ms</td>
		</tr>
		<tr>
			<td align="left">Light sleep</td>
			<td align="right">10 ms</td>
			<td align="right">5 ms</td>
		</tr>
	</table>
</div>
<div class="pt-4 pb-6 text-center text-gray-800 dark:text-gray-100 text-sm italic font-serif">
	Table 1: Time from wake up to successful transmission of small message with ESP-NOW
</div>

<style>
	th {
		font-weight: normal;
	}
	.shadow {
		box-shadow: 6px 6px 0 #00000020;
	}
</style>
