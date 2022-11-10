<script>
	import '../app.css';
	import '../inter.css';
</script>

<div class="flex flex-row justify-between">
	<div class="flex flex-col">
		<a href="/" class="p-4">
			<h1 class="font-bold text-2xl md:text-3xl w-fit">florian</h1>
			<h1 class="font-bold text-2xl md:text-3xl ml-5 -mt-3 w-fit">marending</h1>
		</a>
	</div>
	<div class="flex flex-row gap-6 pr-6 pt-5 md:pt-6">
		<a class="text-sm md:text-md font-semibold h-fit" href="/metrics">metrics</a>
		<a class="text-sm md:text-md font-semibold h-fit" href="/about">about</a>
	</div>
</div>

<slot />

<style global>
	body {
		font-family: 'Inter', sans-serif;
	}
</style>
