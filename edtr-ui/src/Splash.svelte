<script lang="ts">
	import { listen, once, emit } from "@tauri-apps/api/event";
	import { appWindow } from "@tauri-apps/api/window";
	import { onMount } from "svelte";

	let loader: HTMLInputElement;

	listen<number>("splash:progress", (e) => {
		console.log(e.payload);
		loader.value = e.payload.toString();
	});

	once("splash:loaded", () => {
		// TODO: recent projects
		appWindow.close();
	});

	onMount(() => emit("splash:visible"));
</script>

<div class="everything bg-blue-500">
	<div class="loading-text">
		<span class="loading-text-words">L</span>
		<span class="loading-text-words">O</span>
		<span class="loading-text-words">A</span>
		<span class="loading-text-words">D</span>
		<span class="loading-text-words">I</span>
		<span class="loading-text-words">N</span>
		<span class="loading-text-words">G</span>
	</div>
	<input
		type="range"
		min="0"
		max="100"
		value="0"
		class="loading-bar"
		bind:this={loader}
	/>
</div>

<style>
	/* .everything {
		width: 100%;
		height: 100%;
		display: flex;
		flex-direction: column;
		align-items: center;
		background: theme.$primary-background;
		border-radius: 12px;
	}

	.loading-text {
		flex: 1 0 auto;
		display: flex;
		justify-content: center;
		align-items: center;
		width: 100%;
		height: 100px;
		line-height: 100px;
		span {
			display: inline-block;
			margin: 0 5px;
			color: #fff;
			font-family: "Outfit", sans-serif;
			@for $i from 0 through 6 {
				&:nth-child(#{$i + 1}) {
					filter: blur(0px);
					animation: blur-text
						3s
						(calc($i / 5)) +
						s
						infinite
						linear
						alternate;
				}
			}
		}
	}

	.loading-bar {
		pointer-events: none;
		flex: 1 0 auto;
		width: 80%;

		&::-webkit-slider-thumb {
			visibility: hidden;
		}
		&::-moz-range-thumb {
			visibility: hidden;
		}
	}

	@keyframes blur-text {
		0% {
			filter: blur(0px);
		}
		100% {
			filter: blur(4px);
		}
	} */
</style>
