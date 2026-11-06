/* should generate diagnostics */
[...new Set(array)].length;

const foo = new Set([]);
console.log([...foo].length);

function isUnique(array) {
	return[...new Set(array)].length === array.length
}

[...new Set(array),].length;
[...(( new Set(array) ))].length;
(( [...new Set(array)] )).length;
[...new Set].length;
[...new globalThis.Set(array)].length;

bar
;[...new Set(array)].length;

// The fix would remove these comments.
[/* comment */...new Set(array)].length;
[...new Set(array)]/* comment */.length;

// The comment is part of the `Set` expression and is kept.
[...new /* comment */ Set(array)].length;

Array.from(new Set(array)).length;
console.log(Array.from(foo).length);
Array.from((( new Set(array) ))).length;
(( Array.from(new Set(array)) )).length;
Array.from(/* comment */ new Set(array)).length;
Array.from(new /* comment */ Set(array)).length;
globalThis.Array.from(new Set(array)).length;

function isUniqueFrom(array) {
	return Array.from(new Set(array)).length === array.length
}

new [...foo].length.constructor("42");
typeof[...foo].length;
