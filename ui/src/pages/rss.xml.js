import rss from '@astrojs/rss';
import { getCollection } from 'astro:content';

export async function GET() {
	const notes = await getCollection('notes');

	return rss({
		title: 'Florian Marending',
		description: 'Technical musings',
		site: 'https://marending.dev',
		items: notes.map((post) => ({
			title: post.data.title,
			pubDate: post.data.date,
			description: post.data.subtitle,
			link: `/notes/${post.id}/`,
		})),
	});
}
