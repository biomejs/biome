import Component from "@glimmer/component";

// Edge case 1: Array
const templates = [<template><div>First</div></template>, <template><div>Second</div></template>];

// Edge case 2: Object property
const components = {
	Bar: <template><div>Bar</div></template>,
	Baz: <template><div>Baz</div></template>,
};

// Edge case 3: Function argument
registerTemplate(<template><div>Test</div></template>);

// Edge case 4: Return statement
function getTemplate() {
	return <template><div>Test</div></template>;
}

// Edge case 5: Ternary
const template = condition
	? <template><div>A</div></template>
	: <template><div>B</div></template>;
