/* should generate diagnostics */
if (foo === 1) {}
if (foo === 2) {}

{
	if (foo === null) {}
	if (foo === 1) {}
}

{
	if (this === null) {}
	if (this === undefined) {}
}

{
	if (a.b.c === 1) {}
	if (a.b.c === 2) {}
}

switch (x) {
	case "a":
		if (foo === 1) {}
		if (foo === 2) {}
		if (foo === 3) {}
}

{
	if (foo === 1) {
		bar();
	}

	if (foo === 2) {}
}

{
	if (1 === foo) {}
	if (2 === foo) {}
}

{
	if (foo.bar === 1) {}
	if (foo["bar"] === 2) {}
}

{
	if (foo[0] === 1) {}
	if (foo["0"] === 2) {}
}

{
	if (foo === 1 || foo === 2) {}
	if (foo === 3 || foo === 4) {}
}

class Foo {
	#state;

	method() {
		if (this.#state === 1) {}
		if (this.#state === 2) {}
	}

	static {
		if (foo === 1) {}
		if (foo === 2) {}
	}
}

{
	const foo = true;

	if (foo) {}
	if (foo === false) {}
}

{
	if (foo === 1) {}
	else if (foo === 2) {}

	if (foo === 3) {}
	else if (foo === 4) {}

	if (foo === 5) {}
}

{
	if (foo === 1 || foo === 2) {}
	else if (foo === 3) {}

	if (foo === 4) {}
	else if (foo === 5 || foo === 6) {}
}

function handleSignal(process, signal) {
	if (signal === "SIGINT") {
		process.exit(130);
	}

	if (signal === "SIGTERM" || signal === "SIGHUP") {
		process.exit(1);
	}
}

// Changes to other variables, and changes inside functions, don't matter.
{
	if (foo === 1) {
		bar = 1;
		bar++;
		delete bar.baz;
		foo.baz = 1;
		var qux = 1;
		for (bar of values) {}
		[bar = 1, ...bar] = values;
		({ bar } = values);
		callback(() => {
			foo = 2;
		});
		function inner() {
			var foo = 2;
		}
	}
	if (foo === 2) {}
}

{
	if (foo === 1n) {}
	if (foo === 2n) {}
}

{
	if (value === `a`) {}
	if (value === `b`) {}
}

{
	if (value === ``) {}
	if (value === "a") {}
}

{
	if (value === "a") {}
	if (value === 1) {}
}

{
	if (value === "1") {}
	if (value === 1) {}
}

{
	if (object[`state`] === "a") {}
	if (object["state"] === `b`) {}
}

{
	if (foo === 1) {} // first
	if (foo === 2) {}
}

{
	if (foo === 1) {}
	// The second case.
	if (foo === 2) {}
}

{
	if (foo === 1) if (bar) {} else {}
	if (foo === 2) {}
}

{
	if (foo === 1) do if (bar) {} while (bar);
	if (foo === 2) {}
}

{
	if (foo === 1) { if (bar) {} }
	if (foo === 2) {}
}

{
	if (foo === 1) {}
	if (foo === 2) {}
	bar();
	if (foo === 3) {}
}
