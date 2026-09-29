// should generate diagnostics

interface Task {
	id: number;
}

function alwaysTruthyAfterAssignment(x: Task | undefined) {
	x = { id: 1 };
	// `x` is narrowed to the assigned object here, so this condition is always truthy.
	if (x) {
		x;
	}
}

function alwaysFalsyAfterAssignment(x: string | undefined) {
	x = undefined;
	// `x` is narrowed to `undefined` here, so this condition is always falsy.
	if (x) {
		x;
	}
}
