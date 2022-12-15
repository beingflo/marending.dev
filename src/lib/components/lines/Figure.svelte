<script>
	export let caption = '';
</script>

<div class="w-full sm:w-fit mx-auto pb-6 pt-4">
	<slot />
	{#if caption}
		<div class="pt-4 text-center text-gray-800 dark:text-gray-100 text-sm italic font-serif">
			{caption}
		</div>
	{/if}
</div>
