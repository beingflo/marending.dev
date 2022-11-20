<script lang="ts">
	import { enhance } from '$app/forms';

	let email: string;
	let content: string;

	const submit = () => {
		fetch('/api/feedback', { method: 'POST', body: JSON.stringify({ email, content }) });
	};
</script>

<form
	class="fixed bottom-8 right-2 border border-black bg-white p-2 flex flex-col gap-2 left-2 md:left-auto"
	use:enhance
>
	<span class="mb-2 font-semibold">Feedback</span>
	<p class="mb-2 text-gray-600 text-sm">
		Found a typo or want to let me know something?<br />Shoot me a quick message, it's much
		appreciated!
	</p>
	<label class="block">
		<span class="text-sm text-gray-700 mb-1">Email (optional)</span>
		<input
			type="text"
			name="content"
			bind:value={email}
			class="block w-full border border-gray-900 p-0.5"
		/>
	</label>
	<label class="block">
		<span class="text-sm text-gray-700 mb-1">Message</span>
		<textarea
			type="text"
			name="content"
			bind:value={content}
			class="block resize w-full h-24 border border-gray-900 p-0.5"
		/>
	</label>
	<button class="w-fit self-end border border-black py-1 px-2" on:click|preventDefault={submit}
		>Send</button
	>
</form>

<style>
	form {
		box-shadow: 4px 4px 0 rgba(0, 0, 0, 0.15);
	}
</style>
