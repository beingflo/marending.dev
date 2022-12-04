<script>
	export let caption = '';
</script>

<div class="w-fit mx-auto pb-6 pt-4">
	<slot />
	{#if caption}
		<div class="pt-4 text-center text-gray-800 text-sm italic">{caption}</div>
	{/if}
</div>
