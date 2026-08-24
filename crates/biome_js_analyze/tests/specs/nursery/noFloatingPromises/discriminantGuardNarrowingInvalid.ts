// should generate diagnostics

type Task =
	| { kind: "sync" }
	| { kind: "async"; run: () => Promise<void> };

function guardedCall(task: Task) {
	if (task.kind === "async") {
		// `task` is narrowed to the async variant here, so the call floats.
		task.run();
	}
}
