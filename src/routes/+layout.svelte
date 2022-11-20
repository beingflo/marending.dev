<script>
	import Feedback from '$lib/components/Feedback.svelte';
	import { page } from '$app/stores';
	import '../app.css';
	import '../inter.css';

	let showFeedback = false;

	const toggleFeedback = () => (showFeedback = !showFeedback);
</script>

<header class="flex flex-row justify-between h-fit max-w-screen-2xl mx-auto">
	<a href="/" class="flex flex-row gap-3 m-4 h-fit">
		<img alt="icon" class="w-12 h-12 md:w-16 md:h-16" src="/icon.svg" />
		<div class="flex flex-col h-fit">
			<h1 class="font-normal text-2xl md:text-3xl w-fit tracking-tight">florian</h1>
			<h1 class="font-thin text-2xl md:text-3xl ml-5 -mt-3 w-fit tracking-tight">marending</h1>
		</div>
	</a>
	<nav class="grid grid-cols-2 sm:grid-cols-4 gap-2 gap-x-6 sm:gap-6 pr-6 pt-5 md:pt-6 h-fit">
		<a class="text-sm md:text-md h-fit {$page.url.pathname === '/' && 'underline'}" href="/"
			>notes</a
		>
		<a
			class="text-sm md:text-md h-fit {$page.url.pathname === '/images' && 'underline'}"
			href="/images">images</a
		>
		<a
			class="text-sm md:text-md h-fit {$page.url.pathname === '/metrics' && 'underline'}"
			href="/metrics">metrics</a
		>
		<a
			class="text-sm md:text-md h-fit {$page.url.pathname === '/about' && 'underline'}"
			href="/about">about</a
		>
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
