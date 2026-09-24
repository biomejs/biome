// should generate diagnostics

type Task = { run: () => Promise<void> };

function isTask(value: unknown): value is Task {
	return typeof value === "object" && value !== null && "run" in value;
}

function guardedCall(value: unknown) {
	if (isTask(value)) {
		// `value` is narrowed to `Task` here, so the call floats.
		value.run();
	}
}

function isSecondTask(first: unknown, second: unknown): second is Task {
	return typeof second === "object" && second !== null && "run" in second;
}

// The predicate narrows the argument it names, not the first one.
function guardedSecondArgument(first: unknown, second: unknown) {
	if (isSecondTask(first, second)) {
		second.run();
	}
}
