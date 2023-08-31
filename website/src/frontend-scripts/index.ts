import "./mobile";
import "./package-manager-commands";
import "./toc";
import { getCurrentTheme, matchesDark, setCurrentTheme } from "./util";

//# Team list shuffle

/**
 * @template T
 * @param {Array<T>} array
 * @returns {Array<T>}
 */
function randomShuffle<T>(array: T[]): T[] {
	let count = array.length;
	let index;
	while (count) {
		index = Math.floor(Math.random() * count--);
		const temp = array[count]!;
		array[count] = array[index]!;
		array[index] = temp;
	}
	return array;
}

const creditsPeopleLists = document.querySelectorAll(".credits-people-list");
for (const list of creditsPeopleLists) {
	const items = list.querySelectorAll("li");
	for (const li of randomShuffle(Array.from(items))) {
		list.appendChild(li);
	}
}

//# Color scheme switcher

function toggleColorSchemeSwitch(evt: Event) {
	const currentScheme = getCurrentTheme();
	const newScheme = currentScheme === "dark" ? "light" : "dark";

	if (evt.currentTarget instanceof Element) {
		evt.currentTarget.setAttribute(
			"aria-checked",
			String(newScheme === "dark"),
		);
	}

	document.documentElement.classList.add("transition");
	window.localStorage.setItem("data-theme", newScheme);
	setCurrentTheme(newScheme);
	onColorSchemeChange();
}

function onColorSchemeChange() {
	window.dispatchEvent(new Event("colorschemechange"));
}

const colorSchemeSwitcher = document.querySelector(".color-scheme-switch");
if (colorSchemeSwitcher != null) {
	colorSchemeSwitcher.addEventListener("click", toggleColorSchemeSwitch, false);
}

if (matchesDark !== undefined) {
	matchesDark.addEventListener("change", () => {
		onColorSchemeChange();
	});
}
