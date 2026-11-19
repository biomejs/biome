/* should not generate diagnostics */
const local = new Map();
local.get(1);

export function now() {
	return new Date();
}

export const createSet = () => new Set();

export class Store {
	constructor() {
		this.created = new Date();
	}

	get params() {
		return new URLSearchParams();
	}
}

export const value = new Date().getTime();

export class Cache {
	#entries = new Map();
	get(key) {
		return this.#entries.get(key);
	}
}

export class TsLikeCache {
	static #shared = new Set();
}
