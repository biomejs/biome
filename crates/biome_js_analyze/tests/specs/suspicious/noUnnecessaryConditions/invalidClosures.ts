// A read in a closure created after a test sees the test, as in TypeScript,
// when the variable is a `const`, or a parameter or a `let` that nothing
// assigns after the closure is created, so each condition below can't
// change.

interface Timer {
	cancel(): void;
}

declare function isTimer(value: unknown): value is Timer;
declare function next(): Timer | null;

function tested(timer: Timer | null) {
	if (timer !== null) {
		return () => timer?.cancel();
	}
}

function guarded(timer: Timer | null) {
	if (isTimer(timer)) {
		return () => timer?.cancel();
	}
}

function constant() {
	const timer = next();
	if (timer) {
		[1].forEach(() => timer?.cancel());
	}
}

function pastLastAssignment(timer: Timer | null) {
	timer ??= next();
	if (!timer) return;
	return () => timer?.cancel();
}

function method(timer: Timer | null) {
	if (timer) {
		return {
			stop() {
				timer?.cancel();
			},
		};
	}
}
