const colorNames = require("./src/types/ThemeColours.json");

let colors = {};
colorNames.map((name) => {
    colors[name] = `var(--${name})`
});

/** @type {import('tailwindcss').Config} */
module.exports = {
	content: ["./src/**/*.{html,js,svelte,ts}"],
	theme: {
		extend: {
            colors: {
                ...colors
            }
        },
	},
	plugins: [],
};
