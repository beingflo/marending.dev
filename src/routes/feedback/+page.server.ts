import { getFeedback } from '$lib/db';

export async function load() {
	return { feedbacks: getFeedback() };
}
