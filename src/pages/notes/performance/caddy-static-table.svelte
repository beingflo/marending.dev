<div class="shadow mt-4 border border-black dark:border-white p-4 lg:-mx-4">
	<table class="w-full">
		<tr class="border-b border-black dark:boder-white">
			<th align="left">Page</th>
			<th align="right">Requests / second</th>
			<th align="right">Avg. latency</th>
			<th align="right">99% latency</th>
		</tr>
		<tr>
			<td align="left">/</td>
			<td align="right">46'553 rps</td>
			<td align="right">1.6 ms</td>
			<td align="right">24.6 ms</td>
		</tr>
		<tr>
			<td align="left">/notes/lines</td>
			<td align="right">42'973 rps</td>
			<td align="right">2.4 ms</td>
			<td align="right">43.8 ms</td>
		</tr>
	</table>
</div>
<div class="pt-4 pb-6 text-center text-gray-800 dark:text-gray-100 text-sm italic font-serif">
	Table 4: Performance characteristics of Caddy serving static pages built by SvelteKits static
	adapter.
</div>

<style>
	th {
		font-weight: normal;
	}
	.shadow {
		box-shadow: 6px 6px 0 #00000020;
	}
</style>
