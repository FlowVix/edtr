<script lang="ts">
	import { GradientView, RangeView, Split } from "../tree";
	import Tree from "./tree/Tree.svelte";

	let tree = new Split(
		new RangeView(25),
		new Split(
			new RangeView(33.33),
			new GradientView("#ff0000", "#000000"),
			true
		),
		false,
		0.25
	);
</script>

<div class="w-full h-full p-2 bg-transparent">
	<Tree bind:branch={tree} parent={null} />
</div>
