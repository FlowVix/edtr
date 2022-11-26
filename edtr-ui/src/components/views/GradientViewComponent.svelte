<script lang="ts">
	import type { GradientView } from "../../tree";

	export let data: GradientView;
</script>

<div
	class="w-full h-full flex justify-center items-center gap-2"
	style="background: linear-gradient(180deg, {data.top} 0%, {data.bottom} 100%);"
>
	<input type="color" bind:value={data.top} />
	<input type="color" bind:value={data.bottom} />
	ds <br /> fgfgdfgsfd <br />sdfgdfg <br />dfsg
</div>
