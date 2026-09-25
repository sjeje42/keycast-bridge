import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
export default defineConfig({plugins:[svelte()], build:{rollupOptions:{output:{entryFileNames:'assets/overlay.js', assetFileNames:'assets/overlay.[ext]'}}}});
