import { RuleTester } from "@biomejs/runtime/plugin/testing";

new RuleTester().run("context", "tests/plugins/context.ts", {
	valid: [],
	invalid: [
		{
			name: "a TypeScript filename selects a TypeScript module",
			code: "const x: number = 1;",
			filename: "src/example.ts",
			diagnostics: [
				{
					message: /^typescript:module:.*src\/example\.ts$/,
					severity: "information",
				},
			],
		},
		{
			name: "a CommonJS filename selects a script",
			code: "const x = 1;",
			filename: "example.cjs",
			diagnostics: [{ message: /^javascript:script:/ }],
		},
	],
});
