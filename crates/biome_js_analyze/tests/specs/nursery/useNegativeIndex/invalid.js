/* should generate diagnostics */
foo.slice(foo.length - 2, foo.length - 1);
foo.splice(foo.length - 1, 1);
foo.at(foo.length - 1);
foo.subarray(foo.length - 1);
foo.toSpliced(foo.length - 1, 1);
foo.slice(foo.length  -  1);
foo.slice(0, foo.length - 4);
foo?.slice(foo.length - 1);
foo.slice?.(foo.length - 1);
(foo.slice)(foo.length - 1);
this.items.at(this.items.length - 1);

// Larger subtractions
foo.at(foo.length - 2);
foo.slice(foo.length - 3, foo.length - 1);
foo.toSpliced(foo.length - 3, foo.length - 6);
foo.subarray(foo.length - 3, foo.length - 6);

// Nested subtractions
foo.slice(foo.length - 1 - 1);
foo.slice((((foo.length)) - 1) - 1);

// The same receiver written differently
foo.bar.slice(foo.bar.length - 1);
foo["bar"].slice(foo["bar"].length - 1);
foo.bar.slice(foo["bar"].length - 1);
foo[`bar`].slice(foo["bar"].length - 1);
foo[1].slice(foo[1].length - 1);
foo[1].slice(foo["1"].length - 1);

// Comments
foo.slice(/* keep */ foo.length - 1);
foo.slice(foo.length - 1 /* keep */);
foo.slice(foo.length /* lost */ - 1);
foo.slice(foo /* lost */ .length - 1);
foo.slice(
	// comment 1

	foo.length

	// comment 2
	- 1
	-1
	,
	foo.length // comment 3
	- 1
);

// Calls through `call()` and `apply()`
[].slice.call(foo, foo.length - 1, foo.length - 2, foo.length - 3);
[].splice.call(foo, foo.length - 1, foo.length - 2, foo.length - 3);
[].slice.apply(foo, [foo.length - 1, foo.length - 2, foo.length - 3]);
[].splice.apply(foo, [foo.length - 1, foo.length - 2, foo.length - 3]);
[].toSpliced.call(foo, foo.length - 3, foo.length - 6);
''.slice.call(foo, foo.length - 1);
``.slice.call(foo, foo.length - 1);
''.slice.apply(foo, [foo.length - 1]);
Array.prototype.slice.call(foo, foo.length - 2, foo.length - 1);
Array.prototype.slice.apply(foo, [foo.length - 2, foo.length - 1]);
Array.prototype.splice.call(foo, foo.length - 1, 1);
Array.prototype.toSpliced.call(foo, foo.length - 3, foo.length - 6);
Array.prototype.at.call(foo, foo.length - 2);
Array.prototype.at.apply(foo, [foo.length - 3]);
String.prototype.slice.call(foo, foo.length - 1);
String.prototype.at.call(foo, foo.length - 1);
ArrayBuffer.prototype.slice.call(foo, foo.length - 1);
Int8Array.prototype.slice.call(foo, foo.length - 1);
Uint8Array.prototype.subarray.call(foo, foo.length - 3, foo.length - 6);
Uint8Array.prototype.subarray.apply(foo, [foo.length - 3, foo.length - 6]);
Uint8ClampedArray.prototype.at.call(foo, foo.length - 1);
Float16Array.prototype.slice.call(foo, foo.length - 1);
BigUint64Array.prototype.slice.call(foo, foo.length - 1);
globalThis.Array.prototype.slice.call(foo, foo.length - 1);
(Array.prototype.slice).call(foo, foo.length - 1);
Array.prototype.slice.apply(foo, [, foo.length - 1]);
