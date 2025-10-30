import Component from "@glimmer/component";

export default class MyComponent extends Component {
	// Case 1: Class member template (no semicolon wanted)
	<template><div>Member</div></template>

	// Case 2: Assignment inside class (semicolon wanted)
	someMethod() {
		const foo = <template><div>Assignment</div></template>;
		return foo;
	}
}
