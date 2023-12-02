export type ThemeName = "dark" | "light";
export type ThemeChanged = { theme?: string };

export const matchesDark: undefined | MediaQueryList =
	typeof window === "undefined"
		? undefined
		: window.matchMedia("(prefers-color-scheme: dark)");

export function getCurrentTheme(themeName?: string): ThemeName {
	let currentScheme =
		themeName ?? window.localStorage.getItem("starlight-theme");

	if (currentScheme == null || currentScheme === "auto") {
		currentScheme = matchesDark?.matches ? "dark" : "light";
	}
	return currentScheme === "dark" ? "dark" : "light";
}

export function setCurrentTheme(theme: ThemeName) {
	document.documentElement.setAttribute("data-theme", theme);
}
