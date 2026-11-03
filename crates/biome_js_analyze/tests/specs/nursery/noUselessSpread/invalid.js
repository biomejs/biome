/* should generate diagnostics */

// Array literal spread into an array literal
const a1 = [...[a]];
const a2 = [...[a,]];
const a3 = [...[a,],];
const a4 = [...(( [a] ))];
const a5 = [...[]];
const a6 = [...[,]];
const a7 = [...[,,]];
const a8 = [...[a, , b,]];
const a9 = [...[a, , b,],];
const a10 = [a, ...[a, b]];
const a11 = [...[a, b], b,];
const a12 = [a, ...[a, b], b,];
const a13 = [a, ...[], b];
const a14 = [a, ...(( [] )),];
const a15 = [a, ...(( [] ))];
const a16 = [a, ...[b], c];
const a17 = [...[,], b];
const a18 = [
	a,
	...[
		b,
		c,
	],
	d,
];

// Array literal spread into arguments
foo(...[a]);
foo(...[a,]);
foo(...[a,],);
foo(...(( [a] )));
foo(...[]);
foo(...[,]);
foo(...[,,]);
foo(...[a, , b,]);
foo(...[,, ,(( a )), ,,(0, b), ,,]);
foo(a, ...[a, b]);
foo(...[a, b], b,);
new Foo(...[a]);
new Foo(a, ...[a, b], b,);
call(foo, ...[bar, ...baz]);
new Map(...[...iterable]);
new Set(...[iterable]);
Promise.all(...[...iterable]);

// Object literal spread into an object literal
const o1 = {...{a}};
const o2 = {...{a,}};
const o3 = {...{a,},};
const o4 = {...(( {a} ))};
const o5 = {...{}};
const o6 = {a, ...{a, b}};
const o7 = {...{a, b}, b,};
const o8 = {a, ...(({})), b,};
const o9 = {a:1, ...{a: 2}};
const o10 = {...{a:1}, ...{a: 2}};
const o11 = {[a]:1, ...{[a]: 2}};
const o12 = {...{["__proto__"]: p}};
const o13 = {...{__proto__}};
const o14 = {...{read() { return this /* keep */ .value; }}};
const o15 = {
	a: 1,
	...{
		get b() {},
	},
};

// Object.assign() source with only spreads
Object.assign(target, {...source});
Object.assign(target, {...a}, {...b});
Object.assign(target, first, {...second});
Object.assign(...args, target, {...source});
Object.assign(target, {...first, ...second}, third);
Object.assign(target, {...source,}, third);
Object.assign(target, {...(( source ))});
Object.assign(target, {...(foo, bar)});
Object.assign(target, {/* keep */ ...source});

// Spread as the only argument of a collection constructor
new Set(...iterable);
new Map(...iterable);
new WeakSet(...iterable);
new WeakMap(...iterable);
new Set(...getNames());

// Iterable converted to an array
const i1 = new Map([...iterable]);
const i2 = new WeakMap([...iterable]);
const i3 = new Set([...iterable]);
const i4 = new WeakSet([...iterable]);
const i5 = Promise.all([...iterable]);
const i6 = Promise.allSettled([...iterable]);
const i7 = Promise.any([...iterable]);
const i8 = Promise.race([...iterable]);
const i9 = Array.from([...iterable]);
const i10 = BigUint64Array.from([...iterable]);
const i11 = Object.fromEntries([...iterable]);
const i12 = new Map([...iterable,]);
const i13 = new Map([...iterable,],);
const i14 = new Map([...(( iterable ))]);
const i15 = new Map((( [...(( iterable ))] )));
const i16 = Promise.all([.../* keep */ values]);
for (const foo of [...iterable]);
for (const foo of [...iterable,]);
for (const foo of [...(( iterable ))]);
for (const foo of (( [...(( iterable ))] )));
for (const foo of[...iterable]);
for (const value of [...(first || second)]) {}
async () => {for await (const foo of [...iterable]);};
function * f1() {
	yield * [...iterable];
}
function * f2() {
	yield * [...iterable,];
}
function * f3() {
	yield * (( [...iterable] ));
}
function * f4() {
	return yield* [.../* keep */ values];
}

// Copy of a new array
const c1 = [...foo.concat(bar)];
const c2 = [...foo.flat()];
const c3 = [...foo.slice(1)];
const c4 = [...foo.splice(1)];
const c5 = [...foo.toReversed()];
const c6 = [...foo.toSorted()];
const c7 = [...foo.toSpliced(0, 1)];
const c8 = [...foo.with(0, bar)];
const c9 = [...foo.split("|")];
const c10 = [...Object.keys(foo)];
const c11 = [...Object.values(foo)];
const c12 = [...Array.from(foo)];
const c13 = [...Array.of()];
const c14 = [...new Array(3)];
async () => [...await Promise.all(foo)];
async () => [...await Promise.allSettled(foo)];
function r1(bar) {
	return[...Object.keys(bar)];
}
function r2(bar) {
	return[
		...Object.keys(bar)
	];
}
function r3(bar) {
	return[
		...(
			Object.keys(bar)
		)
	];
}
function r4(bar) {
	return([
		...Object.keys(bar)
	]);
}
async () => [...await Promise.all(x)].length;
async () => [...await Promise.all(x)][0];
async () => [...await Promise.all(x)]?.length;
new [...Array.of(1)].constructor(2).length;
new ([...Array.of(1)]).constructor(2);
new [...(Array.of(1))].constructor(2);
[...function () { return [1]; }().toReversed()];
const p1 = () => [...{value: [1]}.value.toReversed()];
const p2 = [...function () { return [1]; }().toReversed()];
const k1 = [a, ...[
	// keep
	b,
]];
foo(
	a,
	...[b, c],
);
function shadowedUndefined(undefined) {
	foo(...[,]);
	foo(...[a]);
}
Object.assign(target, {
	...a,
	...b,
});
const g1 = {...{set b(value) {}}};
