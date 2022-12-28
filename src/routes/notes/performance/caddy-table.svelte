<div class="shadow mt-4 border border-black dark:border-white p-4 lg:-mx-4">
	<table class="w-full">
		<tr class="border-b border-black dark:border-white">
			<th align="left">Page</th>
			<th align="right">Requests / second</th>
			<th align="right">Avg. latency</th>
			<th align="right">99% latency</th>
		</tr>
		<tr>
			<td align="left">/</td>
			<td align="right">6'041 rps</td>
			<td align="right">16 ms</td>
			<td align="right">25 ms</td>
		</tr>
		<tr>
			<td align="left">/notes/lines</td>
			<td align="right">4'342 rps</td>
			<td align="right">22 ms</td>
			<td align="right">31 ms</td>
		</tr>
	</table>
</div>
<div class="pt-4 pb-6 text-center text-gray-800 dark:text-gray-100 text-sm italic font-serif">
	Table 3: Performance characteristics of a SvelteKit node server serving static assets. Requests
	are reverse proxied via Caddy.
</div>

<style>
	th {
		font-weight: normal;
	}
	.shadow {
		box-shadow: 6px 6px 0 #00000020;
	}
</style>
