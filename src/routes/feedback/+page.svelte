<script>
	export let data;
	const { feedbacks } = data;
</script>

{#await feedbacks}
	Loading
{:then feedbacks}
	<div class="p-2 flex flex-col gap-2">
		{#each feedbacks as feedback}
			<p>{feedback.content}</p>
			<p>{feedback.email}</p>
			<p>{feedback.timestamp}</p>
		{/each}
	</div>
{:catch someError}
	System error: {someError.message}.
{/await}
