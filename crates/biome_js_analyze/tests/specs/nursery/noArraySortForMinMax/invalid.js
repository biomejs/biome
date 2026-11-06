/* should generate diagnostics */

const min1 = array.sort((a, b) => a - b)[0];
const min2 = array.toSorted((a, b) => a - b)[0];
const min3 = arrays[key?.name].toSorted((a, b) => a - b)[0];
const min4 = array.sort((a, b) => a - b).at(0);
const min5 = array.toSorted((a, b) => a - b).at(0);
const max1 = array.sort((a, b) => a - b).at(-1);
const max2 = array.toSorted((a, b) => a - b).at(-1);
const max3 = array.sort((a, b) => b - a)[0];
const max4 = array.toSorted((a, b) => b - a)[0];
const min6 = array.sort((a, b) => b - a).at(-1);
const min7 = array.toSorted((a, b) => b - a).at(-1);
const min8 = array.sort(function (a, b) {
	return a - b;
})[0];
const max5 = array.toSorted((a, b) => {
	return a - b;
}).at(-1);
const min9 = (array ?? fallback).toSorted((a, b) => a - b)[0];
const min10 = array.toSorted((a, b) => {
	// Compare numerically.
	return a - b;
})[0];
class ArraySubclass extends Array {
	getMinimum() {
		return super.sort((a, b) => a - b)[0];
	}
}
for (const value of array.sort((a, b) => a - b)[0]) {}
const min11 = new Int8Array(values).toSorted((a, b) => a - b)[0];
const min12 = [3, 1, 2].sort((a, b) => a - b)[0];
const min13 = (array.toSorted((a, b) => a - b))[(0)];
function shadowed(Math) {
	return array.toSorted((a, b) => a - b)[0];
}
const min14 = tag`${value}`.toSorted((a, b) => a - b)[0];
let reassigned = new Set();
reassigned = [3, 1, 2];
const min15 = reassigned.toSorted((a, b) => a - b)[0];
