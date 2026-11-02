import { RuleTester } from "@biomejs/runtime/plugin/testing";

new RuleTester().run("typed", "tests/plugins/typed.ts", {
	valid: ["let x = 1;"],
	invalid: [
		{
			name: "erasable TypeScript plugins retain relative imports",
			code: "debugger;",
			diagnostics: [{ message: "Remove debugger." }],
			output: "",
		},
	],
});
