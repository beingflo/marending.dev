import { Show, createSignal } from 'solid-js';

export const Feedback = () => {
	const [email, setEmail] = createSignal('');
	const [content, setContent] = createSignal('');
	const [notification, setNotification] = createSignal('');

	const submit = async (event) => {
		event.preventDefault();

		const response = await fetch('/api/feedback', {
			method: 'POST',
			body: JSON.stringify({ email: email(), content: content() }),
		}).catch((error) => {
			setNotification('failure');
			throw error;
		});

		if (response.status === 200) {
			setEmail('');
			setContent('');

			setNotification('success');
		} else {
			setNotification('failure');
		}
	};

	return (
		<form class="fixed bottom-8 right-2 border border-black dark:border-white bg-white dark:bg-black p-2 flex flex-col gap-2 left-2 md:left-auto shadow-[4px_4px_0_rgba(0,0,0,0.15)]">
			<span class="mb-2 font-semibold">Feedback</span>
			<p class="mb-2 text-gray-600 dark:text-gray-200 text-sm">
				Found a typo or want to let me know something?
				<br />
				Shoot me a quick message, it's much appreciated!
			</p>
			<label class="block">
				<span class="text-sm text-gray-700 dark:text-gray-200 mb-1">Email (optional)</span>
				<input
					type="text"
					name="content"
					onChange={(e) => setEmail(e.target.value)}
					class="block w-full border border-gray-900 dark:border-gray-200 dark:bg-black p-0.5"
				/>
			</label>
			<label class="block">
				<span class="text-sm text-gray-700 dark:text-gray-200 mb-1">Message</span>
				<textarea
					name="content"
					onChange={(e) => setContent(e.target.value)}
					class="block resize w-full h-24 border border-gray-900 dark:border-gray-200 dark:bg-black p-0.5"
				/>
			</label>
			<div class="flex flex-row justify-end gap-2 items-center">
				<Show
					when={notification() === 'success'}
					fallback={<p class="text-rose-800 dark:text-rose-200">Something went wrong :(</p>}
				>
					<p class="text-green-800 dark:text-green-200">Thanks!</p>
				</Show>
				<button class="border border-black dark:border-white py-1 px-2" onClick={submit}>
					Send
				</button>
			</div>
		</form>
	);
};
