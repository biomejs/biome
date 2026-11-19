// A `var`, a module variable, and a parameter of an enclosing function
// narrow by the tests and writes of the function that reads them, as in
// TypeScript, which assumes a `var` without an initializer assigned there, so
// each condition below can't change.

interface Timer {
	cancel(): void;
}

declare function make(): Timer;
declare function next(): Timer | null;

let current: Timer | null = next();

function hoisted() {
	var timer: Timer | null = make();
	timer?.cancel();
}

function enclosing() {
	if (current !== null) {
		current?.cancel();
	}
}

function writes() {
	current = make();
	current?.cancel();
}

function parameter(timer: Timer | null) {
	return () => {
		if (timer) {
			timer?.cancel();
		}
	};
}

var pending: Timer | null;

function declaredWithoutValue() {
	if (pending !== null) {
		pending?.cancel();
	}
}
