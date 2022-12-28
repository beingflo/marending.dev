<div class="shadow mt-4 border border-black p-4 lg:-mx-4">
	<table class="w-full">
		<tr class="border-b border-black">
			<th align="left">Page</th>
			<th align="right">Requests / second</th>
			<th align="right">Avg. latency</th>
			<th align="right">99% latency</th>
		</tr>
		<tr>
			<td align="left">/</td>
			<td align="right">2095 rps</td>
			<td align="right">44 ms</td>
			<td align="right">88 ms</td>
		</tr>
		<tr>
			<td align="left">/notes/lines</td>
			<td align="right">455 rps</td>
			<td align="right">207 ms</td>
			<td align="right">268 ms</td>
		</tr>
	</table>
</div>
<div class="pt-4 pb-6 text-center text-gray-800 dark:text-gray-100 text-sm italic font-serif">
	Table 1: Performance characteristics of a SvelteKit node server with server-side rendering
	enabled.
</div>

<style>
	th {
		font-weight: normal;
	}
	.shadow {
		box-shadow: 6px 6px 0 #00000020;
	}
</style>
