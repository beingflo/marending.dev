<script>
	import Feedback from '$lib/components/Feedback.svelte';
	import { page } from '$app/stores';
	import '../app.css';
	import { clickOutside } from '$lib/components/utils';

	let showFeedback = false;

	const toggleFeedback = () => (showFeedback = !showFeedback);
</script>

<header class="flex flex-row justify-between h-fit max-w-screen-2xl mx-auto mt-4">
	<a href="/" class="flex flex-row gap-3 ml-4 h-fit">
		<div class="flex flex-col w-fit">
			<span class="text-lg">florian</span>
			<span class="text-lg -mt-3">marending</span>
		</div>
	</a>
	<nav class="flex flex-row gap-3 sm:gap-6 pr-6 h-fit decoration-1">
		<a
			class="text-sm md:text-md {($page.url.pathname.startsWith('/notes') ||
				$page.url.pathname === '/') &&
				'underline'}"
			href="/">notes</a>
		<a
			class="text-sm md:text-md {$page.url.pathname.startsWith('/metrics') && 'underline'}"
			href="/metrics/views">
			metrics</a>
		<a class="text-sm md:text-md {$page.url.pathname === '/about' && 'underline'}" href="/about">
			about</a>
	</nav>
</header>

<main class="max-w-screen-2xl mx-auto">
	<slot />
	{#if showFeedback}
		<div use:clickOutside on:click_outside={() => (showFeedback = false)}>
			<Feedback />
		</div>
	{/if}
</main>

<footer>
	<button
		on:click={toggleFeedback}
		class="fixed p-2 bottom-0 right-2 text-xs font-light text-gray-400">feedback</button>
</footer>

<style global>
	body {
		color: #333;
	}
</style>
