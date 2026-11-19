/* should not generate diagnostics */
// Each optional chain below stays necessary. A destructuring assignment, the
// head of a `for...of` loop, and `delete` write a value that can be nullish,
// as in TypeScript. A write in a field initializer or in an `async` function
// called where it is defined runs at a point that the flow doesn't follow, so
// the variable keeps its declared type in the whole function. TypeScript
// ignores such a write and gives `timer` the type `Timer` in the last two
// functions, but the write runs before the optional chain.

interface Timer {
	cancel(): void;
}

declare function next(): Timer | null;
declare const timers: (Timer | null)[];
declare const enabled: boolean;

function destructured(timer: Timer | null) {
	if (timer !== null) {
		[timer] = [next()];
		timer?.cancel();
	}
}

function deleted(options: { timer?: Timer }) {
	if (options.timer !== undefined) {
		if (enabled) delete options.timer;
		options.timer?.cancel();
	}
}

function iterated(timer: Timer | null) {
	if (timer !== null) {
		for (timer of timers) {
			timer?.cancel();
		}
	}
}

function initialized(timer: Timer | null) {
	if (timer !== null) {
		class Holder {
			static reset = (timer = next());
		}
		timer?.cancel();
		return Holder;
	}
}

function started(timer: Timer | null) {
	if (timer !== null) {
		(async () => {
			timer = next();
		})();
		timer?.cancel();
	}
}
