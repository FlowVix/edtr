<script lang="ts">
	import type { GradientView, RangeView } from "../../tree";

	export let data: RangeView;
</script>

<div
	class="w-full h-full flex justify-center items-center gap-2"
	style="background: linear-gradient(90deg, #007FFF 0%, #007FFF {data.value}%, #7FBFFF {data.value}%, #7FBFFF 100%);"
>
	<input type="range" min="0" max="100" bind:value={data.value} />
</div>
