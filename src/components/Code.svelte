<script lang="ts">
	export let value: string;
	export let caption: string = '';
	export let respectMargin: boolean = false;
</script>

<div class="mb-6">
	<div class="shadow border border-black {respectMargin || 'lg:-mx-4'}">
		<div class="whitespace-pre-wrap p-2 font-mono">
			{value}
		</div>
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
