// should not generate diagnostics
// Each condition below stays necessary: a test outside a closure doesn't
// reach a read inside it of a `let` declared at the top level of a file,
// which another script can write, a function called where it is defined can
// write the variable, and a `var` can be read before its declarator.

interface Timer {
	cancel(): void;
}

declare function make(): Timer;
declare function next(): Timer | null;

let current: Timer | null = next();

function closure() {
	if (current !== null) {
		return () => current?.cancel();
	}
}

function invoked() {
	if (current !== null) {
		(() => {
			current = next();
		})();
		current?.cancel();
	}
}

function beforeDeclarator() {
	timer?.cancel();
	var timer: Timer | null = make();
}
