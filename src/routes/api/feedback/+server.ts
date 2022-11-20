import { addFeedback } from '$lib/db';

export async function POST({ request }: { request: Request }) {
	const { email, content } = await request.json();

	addFeedback(content, email);

	return new Response(null, { status: 200 });
}
