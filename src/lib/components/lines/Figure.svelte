<script>
	export let caption = '';
</script>

<div class="w-fit mx-auto pb-2">
	<slot />
	{#if caption}
		<div class="pt-3 text-center text-gray-800 text-sm italic">{caption}</div>
	{/if}
</div>
