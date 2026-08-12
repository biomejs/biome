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
