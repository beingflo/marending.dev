<script lang="ts">
	import Highlight from 'svelte-highlight';
	import rust from 'svelte-highlight/languages/rust';
	import theme from 'svelte-highlight/styles/stackoverflow-light';

	export let value: string;
</script>

<svelte:head>
	{@html theme}
</svelte:head>

<div class="border border-black mb-6 lg:-mx-4">
	<Highlight language={rust} code={value} />
</div>

<style>
	div {
		box-shadow: 6px 6px 0 lightgray;
	}
</style>
