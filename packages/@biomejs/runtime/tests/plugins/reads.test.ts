import { RuleTester } from "@biomejs/runtime/plugin/testing";

new RuleTester().run("reads", "tests/plugins/reads.ts", {
	valid: ["let x = 1;"],
	invalid: [
		{
			code: "const x = 1; console.log(x);",
			diagnostics: [{ message: "Binding read." }],
		},
	],
});
