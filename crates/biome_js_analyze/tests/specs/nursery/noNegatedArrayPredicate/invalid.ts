/* should generate diagnostics */
const a = !array.some(element => element!.enabled);
const b = !array.some(element => (element as Item).enabled);
const c = !array.some(element => (element satisfies Item).enabled);
const d = !array.some(element => element as unknown);
const e = !array.some(element => element satisfies unknown);
const f = !array.some(element => <boolean>element);
const g = !foo.every(x => x.b)!;
const h = !(foo.every(x => x.b) as boolean);
const i = !(foo.every(x => x.b) satisfies boolean);
function typedArray(array: Int8Array) {
	return !array.some(element => test(element));
}
function* yields(array: unknown[]) {
	yield <boolean>!
		array.every(value => value);
}
