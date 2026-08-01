import rss from '@astrojs/rss';
import { getCollection } from 'astro:content';
import { readFileSync, existsSync } from 'node:fs';
import path from 'node:path';

const MAX_ITEMS = 20;

// The note pages are built as static HTML before this endpoint runs, so we can
// reuse their exact rendered output (charts, code blocks, everything) instead
// of re-rendering MDX here. Falls back to summary-only if a page isn't built
// yet (e.g. `astro dev`).
function articleContent(post) {
	const file = path.join(process.cwd(), 'dist', 'notes', post.id, 'index.html');
	if (!existsSync(file)) return undefined;

	const html = readFileSync(file, 'utf-8');
	const main = html.match(/<main>([\s\S]*?)<\/main>/);
	if (!main) return undefined;

	return main[1]
		.replace(/<a[^>]*href="\/"[^>]*>← back<\/a>/g, '')
		.replace(/<script[\s\S]*?<\/script>/g, '')
		.trim();
}

export async function GET() {
	const notes = (await getCollection('notes'))
		.filter((post) => !post.data.draft)
		.sort((a, b) => new Date(b.data.date) - new Date(a.data.date))
		.slice(0, MAX_ITEMS);

	return rss({
		title: 'Florian Marending',
		description: 'Technical musings',
		site: 'https://marending.dev',
		items: notes.map((post) => ({
			title: post.data.title,
			pubDate: post.data.date,
			description: post.data.subtitle,
			link: `/notes/${post.id}/`,
			content: articleContent(post),
		})),
	});
}
