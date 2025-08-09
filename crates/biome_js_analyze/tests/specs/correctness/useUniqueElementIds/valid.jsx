// should not generate diagnostics;
function Foo() {
	const id = useId();
	return <div id={id}></div>;
}

function Foo() {
	const id = useId();
	return (
		<div id={id}>
			<div>bar</div>
		</div>
	);
}

function Foo() {
	const id = useId();
	return (
		<div>
			<div id={id}>bar</div>
		</div>
	);
}

function Foo() {
	const id = crypto.randomUUID();
	return <div id={id}></div>;
}

function Foo({ id }) {
	return <div id={id}></div>;
}

function Foo() {
	const id = useId();
	return React.createElement("div", { id });
}

function Foo() {
	return createElement("div", { id: "foo" });
}

import { createElement } from "not-react";
function Foo() {
	return createElement("div", { id: "foo" });
}
