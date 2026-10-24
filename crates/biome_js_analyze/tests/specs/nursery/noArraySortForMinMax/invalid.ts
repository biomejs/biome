/* should generate diagnostics */

const min1 = array.sort((a: number, b: number) => a - b)[0];
const min2 = array.toSorted((a, b) => (a - b) as number)[0];
const min3 = array.toSorted((a, b) => a - b satisfies number)[0];
const min4 = array.toSorted((a, b) => a! - b!)[0];
function typed(array: Int8Array) {
	return array.toSorted((a, b) => a - b)[0];
}
function numbers(values: number[]) {
	return values.toSorted((a, b) => a - b).at(-1);
}
function mixed(values: number[] | Set<number>) {
	return values.toSorted((a, b) => a - b)[0];
}
