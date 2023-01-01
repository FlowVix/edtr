import { invoke } from "@tauri-apps/api";

import type { EDTRTheme } from "../types/EDTRTheme";
import colorNames from "../types/ThemeColours.json";

export const setThemeColors = async () => {
	let colors = (await invoke<EDTRTheme>("get_edtrtheme")).colours;

	for (let name of colorNames) {
		console.log(`--${name}`);
		document.documentElement.style.setProperty(`--${name}`, colors[name]);
	}
};
