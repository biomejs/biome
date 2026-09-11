// should generate diagnostics

// `task.run()` floats with or without narrowing; narrowing must not hide it.

type Task =
	| { kind: "sync" }
	| { kind: "async"; run: () => Promise<void> };

function guardedCall(task: Task) {
	if (task.kind === "async") {
		task.run();
	}
}

// A `switch` on the discriminant narrows its cases the same way.
function guardedCallInSwitch(task: Task) {
	switch (task.kind) {
		case "async":
			task.run();
			break;
	}
}
