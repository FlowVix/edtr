<script lang="ts">
	import { appWindow } from "@tauri-apps/api/window";

	import { Router, Route } from "svelte-routing";

	import Close from "./icons/close.svg";
	import Minimise from "./icons/minimize.svg";
	import Maximise from "./icons/maximize.svg";
	import Restore from "./icons/restore.svg";
	import Splash from "./Splash.svelte";
	import Workspace from "./components/Workspace.svelte";

	let maximised = false;
	appWindow.isMaximized().then((m) => (maximised = m));

	appWindow.onResized(() => {
		appWindow.isMaximized().then((m) => (maximised = m));
	});
</script>

<Router url="">
	<Route path="/edtr">
		<div class="w-screen h-screen flex flex-col bg-transparent">
			<div
				data-tauri-drag-region
				class="h-8 bg-red-500 select-none flex items-center justify-end"
			>
				<div class="flex justify-evenly items-center w-40 h-full">
					<div
						class="windows-tb-button"
						on:click={appWindow.minimize}
					>
						<Minimise />
					</div>
					<div
						class="windows-tb-button"
						on:click={() => {
							maximised = !maximised;
							appWindow.toggleMaximize();
						}}
					>
						{#if maximised}
							<Restore />
						{:else}
							<Maximise />
						{/if}
					</div>
					<div
						class="windows-tb-button close"
						on:click={appWindow.close}
					>
						<Close />
					</div>
				</div>
			</div>
			<Workspace />
		</div>
	</Route>
	<Route path="/splash">
		<Splash />
	</Route>
</Router>

<style lang="scss">
	.windows-tb-button {
		@apply flex-1 inline-flex items-center justify-center transition-all h-full;

		&:not(.close):hover {
			background: lighten(blue, 20);
		}

		&.close:hover {
			background: darkred;
		}
	}

	/* // @use "theme";

	// .mac-tb-buttons-container {
	//     top: 1px;
	//     left: 8px;

	//     &:hover,
	//     &:active {
	//         > .mac-tb-button.minimise {
	//             background-color: theme.$macos-minimise;

	//             &:active:hover {
	//                 background-color: darken(theme.$macos-minimise, 10);
	//             }
	//         }
	//         > .mac-tb-button.maximise {
	//             background-color: theme.$macos-maximise;

	//             &:active:hover {
	//                 background-color: darken(theme.$macos-maximise, 10);
	//             }
	//         }
	//         > .mac-tb-button.close {
	//             background-color: theme.$macos-close;

	//             &:active:hover {
	//                 background-color: darken(theme.$macos-close, 10);
	//             }
	//         }
	//     }

	//     > .mac-tb-button {
	//         &:before,
	//         &:after {
	//             visibility: hidden;
	//         }
	//     }

	//     &:hover,
	//     &:active {
	//         > .mac-tb-button {
	//             &:before,
	//             &:after {
	//                 visibility: visible;
	//             }
	//         }
	//     }
	// }

	// .mac-tb-button {
	//     border-radius: 100%;
	//     padding: 0;
	//     height: 12px;
	//     width: 12px;
	//     border: 1px solid rgba(0, 0, 0, 0.06);
	//     box-sizing: border-box;
	//     margin-right: 3.5px;
	//     //background-color: $disabled-gray;
	//     position: relative;
	//     outline: none;

	//     &:before,
	//     &:after {
	//         content: "";
	//         position: absolute;
	//         border-radius: 1px;
	//         left: 0;
	//         top: 0;
	//         right: 0;
	//         bottom: 0;
	//         margin: auto;
	//     }

	//     &.close {
	//         &:before,
	//         &:after {
	//             //background-color: $close-red-icon;
	//             width: 8px;
	//             height: 1px;
	//         }
	//         &:before {
	//             transform: rotate(45deg); // translate(-0.5px, -0.5px);
	//         }
	//         &:after {
	//             transform: rotate(-45deg); // translate(0.5px, -0.5px);
	//         }
	//         &:active:hover:before,
	//         &:active:hover:after {
	//             //background-color: $close-red-icon-active;
	//         }
	//     }

	//     &.minimize {
	//         &:before {
	//             //background-color: $minimize-yellow-icon;
	//             width: 8px;
	//             height: 1px;
	//             //transform: translateY(-0.5px);
	//         }
	//         &:active:hover:before {
	//             //background-color: $minimize-yellow-icon-active;
	//         }
	//     }

	//     &.maximize {
	//         &:before {
	//             // background-color: $maximize-green-icon;
	//             width: 6px;
	//             height: 6px;
	//         }
	//         &:after {
	//             background-color: theme.$macos-maximise;
	//             width: 10px;
	//             height: 2px;
	//             transform: rotate(45deg);
	//         }
	//         &:active:hover:before {
	//             //background-color: $maximize-green-icon-active;
	//         }
	//         &:active:hover:after {
	//             //background-color: $maximize-green-active;
	//         }
	//     }
	// } */

	/* .mac-tb,
	.windows-tb {
		height: 32px;
		background: theme.$titlebar;
		user-select: none;
		display: flex;
		align-items: center;
	}
	.windows-tb {
		justify-content: flex-end;
	}
	.windows-tb-buttons-container {
		display: flex;
		justify-content: space-evenly;
		align-items: center;
		height: inherit;
		width: 10rem;
	}
	.windows-tb-button {
		flex: 1 0 auto;
		height: inherit;
		display: inline-flex;
		align-items: center;
		justify-content: center;

		transition: background 0.1s ease-in-out;

		&:not(.close):hover {
			background: lighten(theme.$titlebar, 20);
		}

		&.close:hover {
			background: theme.$danger;
		}
	}

	.mac-tb {
		justify-content: flex-start;
	}

	.everything {
		width: 100%;
		height: 100%;
		display: flex;
		flex-direction: column;
	}
	.content {
		width: 100%;
		height: 100%;
		display: flex;
		flex-direction: column;
		justify-content: space-evenly;
	} */
</style>
