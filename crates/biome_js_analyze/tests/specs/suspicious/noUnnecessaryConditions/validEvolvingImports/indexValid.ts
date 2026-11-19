/* should not generate diagnostics */
// A `let` initialized with `null` takes the types of the values that the code
// assigns to it, as in TypeScript. Biome can't type a call to an imported
// function whose return type is inferred as TypeScript does, so after such a
// write the variable can hold any value.

import { fetchUser, load } from "./libraryValid";

function loaded() {
	let result = null;
	result = load();
	if (result) {
	}
}

async function fetched() {
	let user = null;
	user = await fetchUser();
	if (user) {
		return user.id;
	}
}
