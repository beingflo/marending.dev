import { sveltekit } from '@sveltejs/kit/vite';
import type { UserConfig } from 'vite';

const config: UserConfig = {
	plugins: [sveltekit()],
	optimizeDeps: {
		include: ['highlight.js', 'highlight.js/lib/core'],
	},
	server: {
		port: 3000,
		strictPort: false,
	},
	preview: {
		port: 4000,
		strictPort: false,
	},
};

export default config;
