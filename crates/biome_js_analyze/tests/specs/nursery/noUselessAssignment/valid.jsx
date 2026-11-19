/* should not generate diagnostics */
function ComponentTag() {
	const A = "";
	return <A />;
}

function ReadInJsx() {
	let A = "";
	foo(A);
	A = "A";
	return <A />;
}

function ReadInAttribute() {
	let x = 0;
	foo(x);
	x = 1;
	return <A prop={x} />;
}

function ReadInSpread() {
	let props = { a: 1 };
	foo(props);
	props = { b: 2 };
	return <A {...props} />;
}

function ReadInBranches() {
	let A;
	if (cond) {
		A = Foo;
	} else {
		A = Bar;
	}
	return <A />;
}
