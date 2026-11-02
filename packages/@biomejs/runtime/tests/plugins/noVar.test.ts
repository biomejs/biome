import { RuleTester } from "@biomejs/runtime/plugin/testing";

new RuleTester().run("noVar", "tests/plugins/noVar.ts", {
	valid: ["let x = 1;"],
	invalid: [
		{
			name: "the safe fix mode leaves unsafe fixes unapplied",
			code: "var café = 1;\n",
			diagnostics: [{ message: /Use let/, severity: "warning" }],
			output: null,
		},
		{
			name: "the unsafe fix mode applies unsafe fixes",
			code: "var café = 1;\n",
			diagnostics: 1,
			output: "let café = 1;\n",
			fixMode: "unsafe",
		},
	],
});
