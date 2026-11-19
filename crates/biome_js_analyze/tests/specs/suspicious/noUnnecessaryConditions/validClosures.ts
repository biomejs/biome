// should not generate diagnostics
// Each condition below stays necessary: the variable is assigned after the
// closure is created or inside a function, the function that reads it is a
// declaration or is created before the test, the variable is a `var`, or only
// `++` assigns it, so the closure may read it before any assignment.

interface Timer {
	cancel(): void;
}

declare function next(): Timer | null;

function writtenLater(timer: Timer | null) {
	if (timer !== null) {
		const stop = () => timer?.cancel();
		timer = next();
		return stop;
	}
}

function writtenInside(timer: Timer | null) {
	if (timer !== null) {
		return () => {
			timer?.cancel();
			timer = next();
		};
	}
}

function declaration(timer: Timer | null) {
	if (timer !== null) {
		function stop() {
			timer?.cancel();
		}
		return stop;
	}
}

function createdBefore(timer: Timer | null) {
	const stop = () => timer?.cancel();
	if (timer !== null) {
		return stop;
	}
}

function hoisted() {
	var timer: Timer | null = next();
	if (timer !== null) {
		return () => timer?.cancel();
	}
}

function incrementedOnly(ready: boolean) {
	let count: number | null;
	if (ready) count++;
	return () => {
		if (count === null) return;
		return count?.toFixed();
	};
}
