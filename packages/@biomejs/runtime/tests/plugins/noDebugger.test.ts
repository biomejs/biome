import { RuleTester } from "@biomejs/runtime/plugin/testing";

new RuleTester().run("noDebugger", "tests/plugins/noDebugger.ts", {
	valid: [
		"let x = 1;",
		{ name: "an empty file", code: "" },
		{ code: "const x: number = 1;", filename: "src/input.ts" },
		{ code: "declare const value: number;", filename: "input.d.ts" },
		{
			name: "a suppression comment",
			code: "// biome-ignore lint/plugin: reason\ndebugger;",
		},
	],
	invalid: [
		{
			code: "debugger;",
			diagnostics: [
				{
					message: "Remove debugger.",
					severity: "error",
					start: { line: 1, column: 1 },
					end: { line: 1, column: 10 },
				},
			],
			output: "",
		},
		{
			name: "columns count code points and CRLF is one newline",
			code: '"😀é"; debugger;\r\ndebugger;',
			diagnostics: [
				{ start: { line: 1, column: 7 }, end: { line: 1, column: 16 } },
				{ start: { line: 2, column: 1 }, end: { line: 2, column: 10 } },
			],
		},
	],
});

new RuleTester({
	config: { javascript: { parser: { jsxEverywhere: false } } },
}).run("noDebugger without JSX everywhere", "tests/plugins/noDebugger.ts", {
	valid: [
		{
			name: "a case configuration replaces the tester configuration",
			code: "const x = <div />;",
			config: { javascript: { parser: { jsxEverywhere: true } } },
		},
	],
	invalid: [],
});
