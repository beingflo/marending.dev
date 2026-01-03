import { defineConfig } from 'astro/config';
import solidJs from '@astrojs/solid-js';
import tailwindcss from '@tailwindcss/vite';
import mdx from '@astrojs/mdx';

import playformCompress from '@playform/compress';

// https://astro.build/config
export default defineConfig({
    integrations: [solidJs(), mdx(), playformCompress()],
    vite: {
        plugins: [tailwindcss()],
    },
});