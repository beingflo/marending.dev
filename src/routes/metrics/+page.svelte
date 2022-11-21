<script>
	import Note from '$lib/components/Note.svelte';

	export let data;
	const { pageViews } = data;
</script>

<Note>
	{#await pageViews}
		Loading...
	{:then pageViews}
		<div class="p-2 flex flex-col gap-1">
			{#each pageViews as pageView}
				<div class="flex flex-row gap-12">
					<p class="font-mono">{pageView.timestamp}</p>
					<p class="font-mono">{pageView.page}</p>
				</div>
			{/each}
		</div>
	{:catch someError}
		System error: {someError.message}.
	{/await}
</Note>
