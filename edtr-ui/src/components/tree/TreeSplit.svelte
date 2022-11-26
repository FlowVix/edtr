<script lang="ts">
	import Tree from "./Tree.svelte";
	import { clamp, map_range } from "../../utils/math";
	import { vec, Vector } from "../../utils/vector";
	import type { Split } from "../../tree";
	export let split: Split;

	let dragOffset: Vector | null = null;
	let dragger: HTMLDivElement;

	const draggerCenterPos = () =>
		vec(
			dragger.offsetLeft + dragger.offsetWidth / 2,
			dragger.offsetTop + dragger.offsetHeight / 2
		);

	let container: HTMLDivElement;
</script>

<svelte:window
	on:pointerup={() => {
		dragOffset = null;
	}}
	on:pointermove={(e) => {
		if (dragOffset != null) {
			let newPos = vec(e.pageX, e.pageY).plus(dragOffset);

			let value = split.vert ? newPos.y : newPos.x;
			let start = split.vert ? container.offsetTop : container.offsetLeft;
			let size = split.vert
				? container.offsetHeight
				: container.offsetWidth;

			let r = clamp(
				map_range(value, start, start + size, 0, 1),
				0.1,
				0.9
			);

			split.ratio = (size * r - 4) / (size - 8);
		}
	}}
/>

<div
	class="w-full h-full flex justify-center items-center"
	class:flex-col={split.vert}
	bind:this={container}
>
	<div
		class="w-full h-full min-w-0 min-h-0 overflow-hidden"
		style:flex={split.ratio}
	>
		<Tree bind:branch={split.a} bind:parent={split} />
	</div>
	<div
		class={split.vert
			? "w-full h-2 cursor-ns-resize"
			: "w-2 h-full cursor-ew-resize"}
		bind:this={dragger}
		on:pointerdown={(e) => {
			dragOffset = draggerCenterPos().minus(vec(e.pageX, e.pageY));
		}}
	/>
	<div
		class="w-full h-full min-w-0 min-h-0 overflow-hidden"
		style:flex={1 - split.ratio}
	>
		<Tree bind:branch={split.b} bind:parent={split} />
	</div>
</div>
