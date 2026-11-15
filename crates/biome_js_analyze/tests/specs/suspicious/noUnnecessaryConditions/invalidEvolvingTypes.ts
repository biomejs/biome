// A `let` declared without an initializer, or with `null` or `undefined`,
// takes its type from the values that the code assigns to it, as in
// TypeScript, so each condition below can't change.

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
