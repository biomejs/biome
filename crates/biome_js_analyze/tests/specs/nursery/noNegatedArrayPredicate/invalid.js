/* should generate diagnostics */
const a = !array.some(element => test(element));
const b = !array.every(element => test(element));
const c = !array.some(element => !test(element));
const d = !array.every(element => !test(element));
const e = !array.some(element => !({}));
const f = !array.some(element => !((foo, bar)));
const g = !array.some((element) => (test(element)));
const h = !array.some(element => ({}));
const i = !array.some(element => element.foo && element.bar);
const j = !array.some(element => element.foo ? element.bar : element.baz);
const k = !array.some(element => /* before */ test(element));
const l = !array.some(element => test(element) /* after */);
const m = !array.some(element => { return test(element); });
const n = !array.some(element => { return !({}); });
const o = !array.every(function (element) { return test(element); });
const p = !array.some(function (element) { return !test(element); }, thisArgument);
const q = !array.some(element => !{ foo: element }.foo);
const r = !array.some(element => () => element);
const s = !array.some(element => element ** 2);
const t = !array.some(element => test(/* comment */ element));
const u = !((array.some(element => test(element))));
const v = !(array.some)(element => test(element));
const w = !array.some((element => test(element)));
const x = !array.some(element => !
	test(element));
if (!array.some(element => test(element))) {}
Boolean(!array.every(element => test(element)));
foo(!array.some(element => test(element)));
!array.some(element => test(element));
foo
!array.some(element => test(element))
const read = () => !{ array: [1] }.array.some(value => value);
const value = !{ array: [1] }.array.some(value => value);
function returns() {
	return!array.some(element => test(element));
}
function returnsOnNextLine() {
	return !
		array.some(element => test(element));
}
function throws() {
	throw!array.some(element => test(element));
}
function throwsOnNextLine() {
	throw !
		array.some(element => test(element)) && ready;
}
function* yields() {
	yield !
		array.every(value => value);
}
function returnsNegatedPredicate() {
	return !array.some(element => { return!test(element); });
}
