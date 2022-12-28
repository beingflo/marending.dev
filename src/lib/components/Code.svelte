<script lang="ts">
	import Highlight from 'svelte-highlight';
	import typescript from 'svelte-highlight/languages/typescript';
	import theme from 'svelte-highlight/styles/stackoverflow-light';

	export let value: string;
	export let language: any = typescript;
	export let respectMargin: boolean = false;
</script>

<svelte:head>
	{@html theme}
</svelte:head>

<div class="border border-black mb-6 {respectMargin || 'lg:-mx-4'}">
	<Highlight {language} code={value} />
</div>

<style>
	div {
		box-shadow: 6px 6px 0 #00000020;
	}
</style>
