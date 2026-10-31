import { getCollection } from 'astro:content';

// A plain `/sitemap.xml`. The site is small and fully static, so the URL list is
// derived from the page files themselves rather than pulled in with
// @astrojs/sitemap, which writes a `sitemap-index.xml` plus chunks.
const SITE = 'https://marending.dev';

// `404.astro` is the miss handler, not a page. The chart pages under
// `notes/league-data/` are pages, so they are included.
const pageFiles = import.meta.glob(['./**/*.astro', '!./404.astro']);

function pagePath(file) {
	// './notes/league-data/MatchesBar.astro' -> '/notes/league-data/MatchesBar/'
	const path = file
		.replace(/^\.\//, '')
		.replace(/\.astro$/, '')
		.replace(/\/index$/, '');
	return path === 'index' ? '/' : `/${path}/`;
}

export async function GET() {
	const notes = (await getCollection('notes'))
		.filter((post) => !post.data.draft)
		// The canonical URL lives in the frontmatter, the collection id is the
		// file path (`league-data/index`).
		.map((post) => post.data.link ?? `/notes/${post.id}/`);

	const urls = [...new Set([...Object.keys(pageFiles).map(pagePath), ...notes])].sort();

	const body = `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
${urls.map((url) => `\t<url><loc>${SITE}${url}</loc></url>`).join('\n')}
</urlset>
`;

	return new Response(body, {
		headers: { 'Content-Type': 'application/xml; charset=utf-8' },
	});
}
