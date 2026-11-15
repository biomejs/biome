// A `let` declared without an initializer and an array initialized with `[]`
// take their types from the values that the code assigns or adds to them, as
// in TypeScript, so each statement below is a promise that nothing handles.

declare function load(): Promise<number>;

function assigned() {
	let pending;
	pending = load();
	pending;
}

function pushed() {
	const pending = [];
	pending.push(load());
	pending;
}
