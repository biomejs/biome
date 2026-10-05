import { Workspace } from "@biomejs/wasm-nodejs";
import { afterEach, beforeEach, describe, expect, it } from "vitest";

describe("GritQL", () => {
	const path = "example.js";
	let workspace: Workspace;
	let projectKey: number;

	beforeEach(() => {
		workspace = new Workspace();
		({ projectKey } = workspace.openProject({
			path: "",
			openUninitialized: true,
		}));
		workspace.openFile({
			projectKey,
			path,
			content: {
				type: "fromClient",
				content: "expect(1).toBeTruthy()",
				version: 0,
			},
		});
	});

	afterEach(() => {
		workspace.free();
	});

	it.each([
		{
			name: "matches an empty call with multiple metavariables",
			pattern:
				"`expect($arg).$method()` where { $arg <: 1, $method <: `toBeTruthy`, }",
		},
		{
			name: "matches an empty call with one metavariable",
			pattern: "`expect($arg).toBeTruthy()` where { $arg <: 1, }",
		},
		{
			name: "matches when the call arguments are captured",
			pattern:
				"`expect($arg).$method($_)` where { $arg <: 1, $method <: `toBeTruthy`, }",
		},
	])("$name", ({ pattern }) => {
		const { patternId } = workspace.parsePattern({
			pattern,
			defaultLanguage: "js",
		});

		try {
			expect(
				workspace.searchPattern({ projectKey, path, pattern: patternId })
					.matches,
			).toEqual([[0, 22]]);
		} finally {
			workspace.dropPattern({ pattern: patternId });
		}
	});
});
