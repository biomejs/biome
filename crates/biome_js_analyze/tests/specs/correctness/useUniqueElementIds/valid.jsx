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

function SvgWithStaticIds() {
	return (
		<svg id="icon">
			<defs>
				<pattern id="pattern" width="10" height="10">
					<path d="M-3 13 15-5" />
				</pattern>
			</defs>
			<rect fill="url(#pattern)" width="100%" height="100%" />
		</svg>
	);
}

function SvgWithCreateElementChild() {
	return (
		<svg>{React.createElement("pattern", { id: "create-element-pattern" })}</svg>
	);
}

function NestedSvgInsideForeignObject() {
	return (
		<svg>
			<foreignObject>
				<svg>
					<pattern id="nested-pattern" width="10" height="10" />
				</svg>
			</foreignObject>
		</svg>
	);
}
