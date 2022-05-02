import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import checker from 'vite-plugin-checker';
import wasmPack from 'vite-plugin-wasm-pack';
import { VitePWA } from 'vite-plugin-pwa';

// https://vitejs.dev/config/
export default defineConfig({
	plugins: [react(), checker({ typescript: true }), wasmPack('../../utils/js_bind'), VitePWA()],
	build: { outDir: 'docs' },
});
