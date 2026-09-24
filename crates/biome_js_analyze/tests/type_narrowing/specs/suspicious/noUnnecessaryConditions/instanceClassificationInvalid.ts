// should generate diagnostics

// The outer guard leaves `undefined`, so the inner condition is always falsy.
function alwaysFalsyPromise(p: Promise<void> | undefined) {
	if (!p) {
		if (p) {
			p;
		}
	}
}

interface Task {
	run(): void;
}

function alwaysFalsyInterface(t: Task | undefined) {
	if (!t) {
		if (t) {
			t;
		}
	}
}
