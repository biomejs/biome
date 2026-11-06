/* should not generate diagnostics */

// The check changes the result
const isNonEmptyAllTruthyArray = array.length > 0 && array.every(Boolean);
const isEmptyArrayOrAllTruthy = array.length === 0 || array.some(Boolean);

// The check and the call aren't next to each other
array.length === 0 || anotherCheck() || array.every(Boolean);
if (
	foo &&
	array.length !== 0 &&
	bar &&
	array.some(Boolean)
) {
	// ...
}

// The check and the call are joined by different operators
(foo && array.length === 0) || array.every(Boolean) && foo;
array.length === 0 || (array.every(Boolean) && foo);
(foo || array.length > 0) && array.some(Boolean);
array.length > 0 && (array.some(Boolean) || foo);

// Not a matching operator
array.length === 0 ?? array.every(Boolean);
array.length === 0 && array.every(Boolean);
(array.length === 0) + (array.every(Boolean));
array.length !== 0 ?? array.some(Boolean);
array.length !== 0 || array.some(Boolean);
array.length > 0 || array.some(Boolean);
array.length === 0 && array.some(Boolean);

// Not a comparison with `0`
array.length === 1 || array.every(Boolean);
array.length === "0" || array.every(Boolean);
array.length === 0. || array.every(Boolean);
array.length === 0x0 || array.every(Boolean);
array.length !== 1 && array.some(Boolean);
array.length > 1 && array.some(Boolean);
array.length >= 0 && array.some(Boolean);
array.length <= 0 && array.some(Boolean);
array.length == 0 || array.every(Boolean);
array.length != 0 && array.some(Boolean);
0 === array.length || array.every(Boolean);
0 !== array.length && array.some(Boolean);
0 < array.length && array.some(Boolean);

// Not a `length` property
array.notLength === 0 || array.every(Boolean);
array[length] === 0 || array.every(Boolean);
array["length"] > 0 && array.some(Boolean);

// Optional chaining
array?.length === 0 || array.every(Boolean);
array.length === 0 || array.every?.(Boolean);
array.length === 0 || array?.every(Boolean);
object?.array.length > 0 && object?.array.some(Boolean);

// Not a call to `every()` or `some()`
array.length === 0 || array.every;
array.length === 0 || array[every](Boolean);
array.length !== 0 && array.notSome(Boolean);
array.length > 0 && array.every(Boolean);
class A {
	#every() {}
	foo() {
		return this.length === 0 || this.#every(Boolean);
	}
}

// Different arrays
array1.length === 0 || array2.every(Boolean);
array1.length > 0 && array2.some(Boolean);
getArray().length > 0 && getArray().some(Boolean);

// Not a single `for...of` loop
if (array.length === 0) { for (const element of array) {} }
if (array.length >= 0) { for (const element of array) {} }
if (array.length > 1) { for (const element of array) {} }
if (array.length != 0) { for (const element of array) {} }
if (array.length > 0 && condition) { for (const element of array) {} }
if (array.length > 0) { for (const element of otherArray) {} }
if (array.length > 0) { before(); for (const element of array) {} }
if (array.length > 0) { for (const element of array) {} after(); }
if (array.length > 0) { for (const element of array) {} } else { fallback(); }
if (array.length > 0) { for (const index in array) {} }
if (array.length > 0) { for (let index = 0; index < array.length; index++) {} }
if (array.length > 0) { while (condition) {} }
async function iterateAsync() {
	if (array.length > 0) { for await (const element of array) {} }
}
if (array?.length > 0) { for (const element of array) {} }
if (array[length] > 0) { for (const element of array) {} }
if (array["length"] > 0) { for (const element of array) {} }

// The loop variable has the same name as the array
if (array.length > 0) { for (const array of array) {} }
if (array.length > 0) { for (const {value: array} of array) {} }
if (object.array.length > 0) { for (const object of object.array) {} }
