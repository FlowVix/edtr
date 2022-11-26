import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import FullReload from "vite-plugin-full-reload";
import wasm from "vite-plugin-wasm";
import { svelteSVG } from "rollup-plugin-svelte-svg";

export default defineConfig({
	plugins: [
		svelte(),
		svelteSVG({ svgo: {}, enforce: "pre" }),
		FullReload(["src/**/*"]),
		wasm(),
	],

	clearScreen: false,
	server: {
		port: 8080,
		strictPort: true,
	},

	envPrefix: ["VITE_", "TAURI_"],
	build: {
		target: ["es2021", "chrome100", "safari13"],
		// don't minify for debug builds
		minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
		// produce sourcemaps for debug builds
		sourcemap: !!process.env.TAURI_DEBUG,
	},
});
