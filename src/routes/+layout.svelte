<script>
	import Feedback from '$lib/components/Feedback.svelte';
	import { page } from '$app/stores';
	import '../app.css';
	import '../font.css';

	let showFeedback = false;

	const toggleFeedback = () => (showFeedback = !showFeedback);
</script>

<header class="flex flex-row justify-between h-fit max-w-screen-2xl mx-auto">
	<a href="/" class="flex flex-row gap-3 m-4 h-fit">
		<div class="flex flex-col h-fit">
			<h1 class="font-semibold lato-light text-2xl md:text-3xl w-fit tracking-tight">florian</h1>
			<h1 class="font-normal lato-light sp text-2xl md:text-3xl ml-5 -mt-3 w-fit tracking-tight">
				marending
			</h1>
		</div>
	</a>
	<nav class="flex flex-row gap-3 sm:gap-6 pr-6 pt-5 md:pt-6 h-fit">
		<a class="text-sm md:text-md {$page.url.pathname === '/' && 'underline'}" href="/">notes</a>
		<a
			class="text-sm md:text-md {$page.url.pathname === '/metrics' && 'underline'}"
			href="/metrics">
			metrics</a>
		<a class="text-sm md:text-md {$page.url.pathname === '/about' && 'underline'}" href="/about">
			about</a>
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
		class="fixed p-2 bottom-0 right-2 text-xs font-light text-gray-400">feedback</button>
</footer>

<style global>
	body {
		font-family: 'LatoLatinWeb', serif;
	}
	.lato-light {
		font-family: 'LatoLatinWebLight', sans-serif;
	}
	.lato {
		font-family: 'LatoLatinWeb', sans-serif;
	}
</style>
