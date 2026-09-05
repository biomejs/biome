// should generate diagnostics

// `task.run()` floats with or without narrowing. These cases exist to catch
// narrowing suppressing the diagnostic, not to prove that narrowing happens --
// the inferred types are pinned by
// `biome_module_graph/tests/spec_tests/narrowing.test.rs`.

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
