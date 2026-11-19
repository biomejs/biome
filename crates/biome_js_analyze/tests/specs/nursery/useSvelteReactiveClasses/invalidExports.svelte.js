/* should generate diagnostics */
const variable1 = new Date(8.64e15);
const variable2 = new Map([[1, "one"], [2, "two"]]);
const variable3 = new Set([1, 2, 1, 3, 3]);
const variable4 = new URLSearchParams("foo=1&bar=2");
const variable5 = new URL("https://svelte.dev/");
const variable6 = new Date(8.64e15);

export { variable1, variable2, variable3, variable4, variable5, variable6 as var };

export const variable7 = new Date(8.64e15);

export const store = { items: new Map(), tags: [new Set()] };

export class Store {
	items = new Map();
}

class Hidden {
	static cache = new Map();
}
export { Hidden };
