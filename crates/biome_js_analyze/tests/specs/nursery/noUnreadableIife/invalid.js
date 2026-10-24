/* should generate diagnostics */
const foo1 = (() => (a ? b : c))();
const foo2 = (() => (
	a ? b : c
))();
const foo3 = (
	() => (
		a ? b : c
	)
)();
const foo4 = (() => (/* comment */ a ? b : c))();
const foo5 = (() => (
	a, b
))();
const foo6 = (() => ({
	a: b,
}))();
const foo7 = (bar => (bar))();
(async () => ({
	bar,
}))();
const foo8 = (async (bar) => ({
	bar: await baz(),
}))();
(async () => (( {bar} )))();
const foo9 = (() => (a ? b : c /* comment */))();
const foo10 = (() => (a /* comment */ ? b : c))();
const foo11 = (() => (
	// comment
	a
))();
const foo12 = (() => (
	a // comment
))();
const foo13 = (() => ((/* comment */ a)))();
const foo14 = ((() => (a)))();
const foo15 = (() => (a))?.();
const foo16 = (() => (a))(b)(c);
const foo17 = (() =>
	// comment
	(a))();
const foo18 = (() => (a) /* comment */)();
