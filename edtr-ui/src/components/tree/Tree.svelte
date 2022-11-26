<script lang="ts">
	import GradientViewComponent from "../views/GradientViewComponent.svelte";
	import RangeViewComponent from "../views/RangeViewComponent.svelte";
	import TreeSplit from "./TreeSplit.svelte";
	import { vec, type Vector } from "../../utils/vector";
	import {
		GradientView,
		RangeView,
		Split,
		View,
		type Branch,
	} from "../../tree";
	export let branch: Branch;
	export let parent: Branch | null;
</script>

{#if branch instanceof Split}
	<TreeSplit bind:split={branch} />
{:else}
	<div class="w-full h-full flex flex-col overflow-hidden rounded-lg">
		<div class="h-6 max-h-6 shrink-0 bg-neutral-700 text-white flex">
			{#if parent != null}
				<button
					on:click={() => {
						if (parent instanceof Split) {
							let other =
								parent.a === branch ? parent.b : parent.a;
							parent = other;
						}
					}}
				>
					<img
						src="close.svg"
						alt="close"
						class="h-6"
						draggable="false"
					/>
				</button>
			{/if}
			<button
				on:click={() => {
					if (branch instanceof View) {
						branch = new Split(branch, branch.duplicate(), false);
					}
				}}
			>
				<img
					src="split_vert.svg"
					alt="split vertically"
					class="h-6"
					draggable="false"
				/>
			</button>
			<button
				on:click={() => {
					if (branch instanceof View) {
						branch = new Split(branch, branch.duplicate(), true);
					}
				}}
			>
				<img
					src="split_horiz.svg"
					alt="split horizontally"
					class="h-6"
					draggable="false"
				/>
			</button>
		</div>
		{#if branch instanceof GradientView}
			<GradientViewComponent bind:data={branch} />
		{:else if branch instanceof RangeView}
			<RangeViewComponent bind:data={branch} />
		{/if}
	</div>
{/if}
