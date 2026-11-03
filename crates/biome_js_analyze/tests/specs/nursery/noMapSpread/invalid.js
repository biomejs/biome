/* should generate diagnostics */

// Object and array spreads returned directly
b.map((x) => ({ ...x }));
b.map((x) => ({ ...x, ...y }));
b.map((x) => {
	return { ...x };
});
b.map((x) => [...x]);
b.map((x) => [...x, ...y]);
b.map(function (x) {
	return [...x, 1];
});

// Method variations
a.map?.((x) => ({ ...x }));
b.flatMap((x) => ({ ...x }));
b["map"]((x) => ({ ...x }));
foo().map((x) => ({ ...x }));
foo[1].map((x) => ({ ...x }));
foo?.bar?.map((x) => ({ ...x }));
(foo ?? bar).map((x) => ({ ...x }));
obj.#foo.map((x) => ({ ...x }));

// The array is never read again
let unused = [];
unused.map((x) => ({ ...x }));

// The array is read before the call, but not after
let readBefore = [];
console.log(readBefore);
readBefore.map((x) => ({ ...x }));

// Spread arguments
b.map((x) => ({ ...x.y }));
b.map((x) => ({ ...x[y] }));
b.map((x) => ({ ...(x ?? y) }));

// Variables declared in the callback
b.map((x) => {
	let x2 = { ...x };
	return x2;
});
b.map((x) => {
	let x2 = { ...x };
	let x3 = x2;
	return x3;
});
b.map((x) => {
	let y = { ...x };
	if (y.foo) {
		return y;
	} else {
		return x;
	}
});
result.map((obj) => {
	const { a = { ...obj }, b = a } = obj;
	return b;
});
result.map((obj) => {
	const [first = [...obj]] = obj;
	return first;
});

// Conditional, logical, and sequence expressions
b.map((x) => (someCond ? { ...x, foo: true } : { ...x, foo: false }));
b.map(({ x, y }) => ({ ...(cond ? x : y) }));
b.map((x, i) => (y ? { ...x, i } : x));
b.map((x, i) => (y ? x : { ...x, i }));
b.map((x) => (0, { ...x }));
b.map(({ x, y }) => x ?? { ...y });

// Fixable: the object starts with a spread
b.map(({ x, y }) => ({ ...x, ...y }));
b.map(({ x, y }) => {
	return { ...x, ...y };
});
b.map(({ x, y }) => ({ ...x, y }));
b.map(({ x, y, z }) => ({ ...x, y, z }));
b.map(({ x, y, z }) => ({ ...x, y, ...z, w: 1 }));

// Not fixable: the object doesn't start with a spread
b.map(({ x, y }) => ({ x, ...y }));
b.map(({ x, y, z }) => ({ x, y, ...z }));

// Not fixable: the object has comments
b.map(({ x, y }) => ({ ...x /* keep */, y }));

// Not fixable: a comment inside the parentheses would be lost
b.map(({ x, y }) => (/* keep */ { ...x, y }));

// Calls aren't followed to the function, even when it's called again
function getItems() {}
getItems().map((x) => ({ ...x }));
getItems();
