<script lang="ts">
	import Highlight from 'svelte-highlight';
	import typescript from 'svelte-highlight/languages/typescript';
	import theme from 'svelte-highlight/styles/stackoverflow-light';

	export let value: string;
	export let caption: string = '';
	export let language: any = typescript;
	export let respectMargin: boolean = false;
</script>

<svelte:head>
	{@html theme}
</svelte:head>

<div class="mb-6">
	<div class="shadow border border-black {respectMargin || 'lg:-mx-4'}">
		<Highlight {language} code={value} />
	</div>
	{#if caption}
		<div class="pt-4 text-center text-gray-800 dark:text-gray-100 text-sm italic font-serif">
			{caption}
		</div>
	{/if}
</div>

<style>
	.shadow {
		box-shadow: 6px 6px 0 #00000020;
	}
</style>
