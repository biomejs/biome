function Foo() {
	return <div id="foo"></div>;
}

function Foo() {
	return (
		<div id="foo">
			<div>bar</div>
		</div>
	);
}

function Foo() {
	return (
		<div>
			<div id="foo">bar</div>
		</div>
	);
}

function Foo() {
	return React.createElement("div", { id: "foo" });
}

import { createElement } from "react";
function Foo() {
	return createElement("div", { id: "foo" });
}

function HtmlInsideForeignObject() {
	return (
		<svg>
			<foreignObject>
				<div id="foo"></div>
			</foreignObject>
		</svg>
	);
}

function CreateElementInsideSvgCallback() {
	return (
		<svg
			onClick={() => React.createElement("div", { id: "foo" })}
		></svg>
	);
}

function JsxElementInsideSvgCallback() {
	return <svg onClick={() => <div id="foo"></div>}></svg>;
}
