import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import wasmPack from 'vite-plugin-wasm-pack';

export default defineConfig({
	build: {
		minify: false,
		target: 'esnext'
	},
	plugins: [sveltekit(), wasmPack(['./rust'])],
	server: {
		fs: {
			allow: ['./rust/pkg/']
		},
		port: 3000
	}
});
