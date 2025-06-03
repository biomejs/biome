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

// SVG elements should be allowed to have static id attributes
function SvgIcon() {
	return (
		<svg viewBox="0 0 24 24">
			<circle id="circle1" cx="12" cy="12" r="10" />
			<path id="path1" d="M12 2L2 7v10l10 5 10-5V7z" />
			<rect id="rect1" x="10" y="10" width="4" height="4" />
		</svg>
	);
}

function ComplexSvg() {
	return (
		<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
			<defs>
				<linearGradient id="gradient1">
					<stop id="stop1" offset="0%" stopColor="red" />
					<stop id="stop2" offset="100%" stopColor="blue" />
				</linearGradient>
				<filter id="filter1">
					<feGaussianBlur id="blur1" stdDeviation="2" />
				</filter>
			</defs>
			<g id="group1">
				<circle id="circle2" cx="50" cy="50" r="20" fill="url(#gradient1)" />
				<text id="text1" x="50" y="50">Hello</text>
			</g>
		</svg>
	);
}

// React.createElement with SVG elements should also be allowed
function SvgWithCreateElement() {
	return React.createElement("svg", { viewBox: "0 0 24 24" },
		React.createElement("circle", { id: "circle3", cx: "12", cy: "12", r: "10" })
	);
}
