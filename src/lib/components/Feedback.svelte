<script lang="ts">
	let email: string;
	let content: string;

	let notification = '';

	const submit = async () => {
		const response = await fetch('/api/feedback', {
			method: 'POST',
			body: JSON.stringify({ email, content }),
		}).catch((error) => {
			notification = 'failure';
			throw error;
		});

		if (response.status === 200) {
			email = '';
			content = '';

			notification = 'success';
		} else {
			notification = 'failure';
		}
	};
</script>

<form
	class="fixed bottom-8 right-2 border border-black dark:border-white bg-white dark:bg-black p-2 flex flex-col gap-2 left-2 md:left-auto">
	<span class="mb-2 font-semibold">Feedback</span>
	<p class="mb-2 text-gray-600 dark:text-gray-200 text-sm">
		Found a typo or want to let me know something?<br />Shoot me a quick message, it's much
		appreciated!
	</p>
	<label class="block">
		<span class="text-sm text-gray-700 dark:text-gray-200 mb-1">Email (optional)</span>
		<input
			type="text"
			name="content"
			bind:value={email}
			class="block w-full border border-gray-900 dark:border-gray-200 dark:bg-black p-0.5" />
	</label>
	<label class="block">
		<span class="text-sm text-gray-700 dark:text-gray-200 mb-1">Message</span>
		<textarea
			type="text"
			name="content"
			bind:value={content}
			class="block resize w-full h-24 border border-gray-900 dark:border-gray-200 dark:bg-black p-0.5" />
	</label>
	<div class="flex flex-row justify-end gap-2 items-center">
		{#if notification === 'success'}
			<p class="text-green-800 dark:text-green-200">Thanks!</p>
		{/if}
		{#if notification === 'failure'}
			<p class="text-rose-800 dark:text-rose-200">Something went wrong :(</p>
		{/if}
		<button class="border border-black dark:border-white py-1 px-2" on:click|preventDefault={submit}
			>Send</button>
	</div>
</form>

<style>
	form {
		box-shadow: 4px 4px 0 rgba(0, 0, 0, 0.15);
	}
</style>
