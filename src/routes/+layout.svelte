<script>
	import Feedback from '$lib/components/Feedback.svelte';
	import '../app.css';
	import '../inter.css';

	let showFeedback = false;

	const toggleFeedback = () => (showFeedback = !showFeedback);
</script>

<header class="flex flex-row justify-between max-w-screen-2xl mx-auto">
	<div class="flex flex-col">
		<a href="/" class="m-4">
			<h1 class="font-bold text-2xl md:text-3xl w-fit tracking-tight">florian</h1>
			<h1 class="font-bold text-2xl md:text-3xl ml-5 -mt-3 w-fit tracking-tight">marending</h1>
		</a>
	</div>
	<nav class="flex flex-row gap-6 pr-6 pt-5 md:pt-6">
		<a class="text-sm md:text-md font-semibold h-fit" href="/metrics">metrics</a>
		<a class="text-sm md:text-md font-semibold h-fit" href="/about">about</a>
	</nav>
</header>

<main class="max-w-screen-2xl mx-auto">
	<slot />
	{#if showFeedback}
		<Feedback />
	{/if}
</main>

<footer>
	<button
		on:click={toggleFeedback}
		class="fixed p-2 bottom-0 right-2 text-xs font-light text-gray-400">feedback</button
	>
</footer>

<style global>
	body {
		font-family: 'Inter', sans-serif;
	}
</style>
