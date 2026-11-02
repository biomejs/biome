import { RuleTester } from "@biomejs/runtime/plugin/testing";

new RuleTester().run("stateful", "tests/plugins/stateful.ts", {
	valid: [],
	invalid: [
		{
			name: "the first case",
			code: "let x = 1;",
			diagnostics: [{ message: "Invocation 1", severity: "hint" }],
		},
		{
			name: "a later case starts with fresh module state",
			code: "let x = 1;",
			diagnostics: [{ message: "Invocation 1", severity: "hint" }],
		},
	],
});
