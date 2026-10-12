/* should not generate diagnostics */
const foo = undefined;

const { bar = undefined } = baz;

[quux = undefined] = quuux;

(foo = undefined) => {};

class Foo {
	bar = undefined;
}

// Exported variables should not be flagged.
// In frameworks like Svelte 4, exported variables with undefined initialization
// are used to declare optional props.

// Direct export
export let directExport = undefined;

// Named export
let namedExport = undefined;
export { namedExport };

// Renamed export (Svelte 4 pattern for reserved words like "class")
let className = undefined;
export { className as class };

// Export default
let defaultExport = undefined;
export default defaultExport;

// var in loops is not useless because removing `= undefined` prevents resetting on subsequent iterations
for (let i = 0; i < 3; i++) {
	var inFor = undefined;
}

for (const x of [1, 2]) {
	var inForOf = undefined;
}

for (const k in { a: 1 }) {
	var inForIn = undefined;
}

while (false) {
	var inWhile = undefined;
}

do {
	var inDoWhile = undefined;
} while (false);

// var redeclarations or parameter resets are not useless
function f1() {
	var z = 1;
	var z = undefined;
}

function f2(param) {
	var param = undefined;
}

