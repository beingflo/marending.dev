<script>
	import Note from '$lib/components/Note.svelte';

	export let data;
	const { feedbacks } = data;
</script>

<Note>
	{#await feedbacks}
		Loading...
	{:then feedbacks}
		<div class="p-2 flex flex-col gap-12">
			{#each feedbacks as feedback}
				<div class="flex flex-col">
					<p class="font-semibold text-sm">{feedback.email || 'unknown'}</p>
					<p class="text-gray-500 text-sm">
						{feedback.timestamp}
					</p>
					<p>{feedback.content}</p>
				</div>
			{/each}
		</div>
	{:catch someError}
		System error: {someError.message}.
	{/await}
</Note>
