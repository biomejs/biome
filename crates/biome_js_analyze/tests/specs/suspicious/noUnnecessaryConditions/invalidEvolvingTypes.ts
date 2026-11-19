// A `let` declared without an initializer, or with `null` or `undefined`,
// takes its type from the values that the code assigns to it, as in
// TypeScript, and is `undefined` before any of them, and an array that starts
// empty takes the types of the values added to it, so each condition below
// can't change.

interface Timer {
	cancel(): void;
}

declare function make(): Timer;

function assigned() {
	let timer;
	timer = make();
	timer?.cancel();
}

function initializedWithNull() {
	let timer = null;
	timer = make();
	timer?.cancel();
}

function clearedOnly() {
	let timer;
	timer = null;
	if (timer) {
	}
}

function neverAssigned() {
	let timer;
	if (timer) {
	}
}

function onlyNulls() {
	const slots = [];
	slots.push(null);
	if (slots[0]) {
	}
}
