import { defineCollection } from 'astro:content';

import { glob } from 'astro/loaders';

const notes = defineCollection({
	loader: glob({ pattern: '**/*.mdx', base: './src/pages/notes' }),
});

export const collections = { notes };
