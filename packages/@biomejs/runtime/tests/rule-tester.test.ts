import assert from "node:assert/strict";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { test } from "node:test";

import type { Workspace } from "@biomejs/backend-jsonrpc";
import {
	type InvalidCase,
	RuleExecutionError,
	RuleTester,
	type RuleTesterOptions,
	type RuleTestSuite,
} from "@biomejs/runtime/plugin/testing";

import {
	createWorkspaceConnection,
	WorkspaceRpcError,
} from "../lib/plugin/testing/jsonrpc.js";

const isLoadError = (error: unknown) =>
	error instanceof RuleExecutionError && error.phase === "load";

const isParseError = (error: unknown) =>
	error instanceof RuleExecutionError && error.phase === "parse";

interface RegisteredCase {
	title: string;
	run: () => Promise<void>;
}

/**
 * Registers a suite with stand-in runner hooks, so that its cases can be run
 * and inspected directly. Disposing it runs the registered `afterAll` hooks.
 */
function register(
	plugin: string | URL,
	cases: RuleTestSuite,
	options: RuleTesterOptions = {},
) {
	const registered: RegisteredCase[] = [];
	const cleanups: (() => void)[] = [];
	const groups: string[] = [];
	const { describe, it, afterAll } = RuleTester;

	RuleTester.describe = (name, fn) => {
		groups.push(name);
		fn();
		groups.pop();
	};
	RuleTester.it = (name, fn) => {
		registered.push({ title: [...groups, name].join(" > "), run: fn });
	};
	RuleTester.afterAll = (fn) => {
		cleanups.push(fn);
	};

	try {
		new RuleTester(options).run("suite", plugin, cases);
	} finally {
		RuleTester.describe = describe;
		RuleTester.it = it;
		RuleTester.afterAll = afterAll;
	}

	return {
		cases: registered,
		[Symbol.dispose]() {
			for (const cleanup of cleanups) {
				cleanup();
			}
		},
	};
}

test("cases are registered under valid and invalid groups", () => {
	// Act
	using suite = register("tests/plugins/noDebugger.ts", {
		valid: ["let x = 1;"],
		invalid: [{ code: "debugger;", diagnostics: 1 }],
	});

	// Assert
	assert.deepEqual(
		suite.cases.map(({ title }) => title),
		["suite > valid > let x = 1;", "suite > invalid > debugger;"],
	);
});

test("a case name replaces the code in the title", () => {
	// Act
	using suite = register("tests/plugins/noDebugger.ts", {
		valid: [{ name: "a declaration", code: "let x = 1;" }],
		invalid: [],
	});

	// Assert
	assert.deepEqual(
		suite.cases.map(({ title }) => title),
		["suite > valid > a declaration"],
	);
});

test("long and empty code have readable titles", () => {
	// Act
	using suite = register("tests/plugins/noDebugger.ts", {
		valid: ["x".repeat(100), ""],
		invalid: [],
	});

	// Assert
	assert.deepEqual(
		suite.cases.map(({ title }) => title),
		[`suite > valid > ${"x".repeat(79)}…`, "suite > valid > (empty)"],
	);
});

test("an empty suite is rejected", () => {
	// Act
	const act = () =>
		register("tests/plugins/noDebugger.ts", { valid: [], invalid: [] });

	// Assert
	assert.throws(act, /contain a case/);
});

test("the suite disposes its connection after its cases", async () => {
	// Arrange
	const suite = register("tests/plugins/noDebugger.ts", {
		valid: ["let x = 1;"],
		invalid: [],
	});
	const [testCase] = suite.cases;

	// Act
	suite[Symbol.dispose]();

	// Assert
	await assert.rejects(testCase.run(), /disposed/);
});

test("plugins load from file URLs", async () => {
	// Arrange
	using suite = register(new URL("./plugins/noDebugger.ts", import.meta.url), {
		valid: ["let x = 1;"],
		invalid: [],
	});
	const [testCase] = suite.cases;

	// Act
	const result = testCase.run();

	// Assert
	await assert.doesNotReject(result);
});

test("the tester parser configuration applies to cases", async () => {
	// Arrange
	using suite = register(
		"tests/plugins/noDebugger.ts",
		{ valid: ["const x = <div />;"], invalid: [] },
		{ config: { javascript: { parser: { jsxEverywhere: false } } } },
	);
	const [testCase] = suite.cases;

	// Act
	const result = testCase.run();

	// Assert
	await assert.rejects(result, isParseError);
});

test("the tester configuration applies again after an overriding case", async () => {
	// Arrange
	using suite = register(
		"tests/plugins/noDebugger.ts",
		{
			valid: [
				{
					code: "const x = <div />;",
					config: { javascript: { parser: { jsxEverywhere: true } } },
				},
				"const x = <div />;",
			],
			invalid: [],
		},
		{ config: { javascript: { parser: { jsxEverywhere: false } } } },
	);
	const [overriding, defaulted] = suite.cases;
	await overriding.run();

	// Act
	const result = defaulted.run();

	// Assert
	await assert.rejects(result, isParseError);
});

const incorrectCases: [string, object, RegExp][] = [
	["count", { diagnostics: 2 }, /diagnostic count/],
	["message", { diagnostics: [{ message: "wrong" }] }, /message/],
	["severity", { diagnostics: [{ severity: "warning" }] }, /severity/],
	["start", { diagnostics: [{ start: { line: 2, column: 1 } }] }, /start/],
	["end", { diagnostics: [{ end: { line: 1, column: 1 } }] }, /end/],
	["output", { diagnostics: 1, output: "wrong" }, /fix output/],
	["unchanged output", { diagnostics: 1, output: null }, /fix output/],
	["empty diagnostics", { diagnostics: [] }, /nonempty diagnostics/],
	["zero diagnostics", { diagnostics: 0 }, /positive integer/],
	[
		"unknown expectation",
		{ diagnostics: [{ rule: "x" }] },
		/unsupported field/,
	],
];
for (const [name, expectation, pattern] of incorrectCases) {
	test(`an incorrect ${name} fails the case`, async () => {
		// Arrange
		using suite = register("tests/plugins/noDebugger.ts", {
			valid: [],
			invalid: [{ code: "debugger;", ...expectation } as InvalidCase],
		});
		const [testCase] = suite.cases;

		// Act
		const result = testCase.run();

		// Assert
		await assert.rejects(result, pattern);
	});
}

test("diagnostics fail a valid case", async () => {
	// Arrange
	using suite = register("tests/plugins/noDebugger.ts", {
		valid: ["debugger;"],
		invalid: [],
	});
	const [testCase] = suite.cases;

	// Act
	const result = testCase.run();

	// Assert
	await assert.rejects(result, /valid case reported diagnostics/);
});

test("a rule crash fails a valid case", async () => {
	// Arrange
	using suite = register("tests/plugins/crashes.ts", {
		valid: ["let x = 1;"],
		invalid: [],
	});
	const [testCase] = suite.cases;

	// Act
	const result = testCase.run();

	// Assert
	await assert.rejects(result, /Deliberate rule crash/);
});

test("a plugin that throws during initialization fails to load", async () => {
	// Arrange
	using suite = register("tests/plugins/load-error.ts", {
		valid: ["let x = 1;"],
		invalid: [],
	});
	const [testCase] = suite.cases;

	// Act
	const result = testCase.run();

	// Assert
	await assert.rejects(result, isLoadError);
});

test("a missing plugin fails to load", async () => {
	// Arrange
	using suite = register("tests/plugins/missing.ts", {
		valid: ["let x = 1;"],
		invalid: [],
	});
	const [testCase] = suite.cases;

	// Act
	const result = testCase.run();

	// Assert
	await assert.rejects(result, isLoadError);
});

test("malformed source is a parse error", async () => {
	// Arrange
	using suite = register("tests/plugins/noDebugger.ts", {
		valid: [],
		invalid: [{ code: "const =", diagnostics: 1 }],
	});
	const [testCase] = suite.cases;

	// Act
	const result = testCase.run();

	// Assert
	await assert.rejects(result, isParseError);
});

test("a case runs after an execution error in a previous case", async () => {
	// Arrange
	using suite = register("tests/plugins/noDebugger.ts", {
		valid: ["const =", "let x = 1;"],
		invalid: [],
	});
	const [failing, passing] = suite.cases;
	await failing.run().catch(() => {});

	// Act
	const result = passing.run();

	// Assert
	await assert.doesNotReject(result);
});

test("unknown case fields are rejected", async () => {
	// Arrange
	using suite = register("tests/plugins/noDebugger.ts", {
		// @ts-expect-error Unknown fields are rejected at runtime too.
		valid: [{ code: "", options: {} }],
		invalid: [],
	});
	const [testCase] = suite.cases;

	// Act
	const result = testCase.run();

	// Assert
	await assert.rejects(result, /unsupported field options/);
});

for (const filename of ["input.vue", "../input.js", "/tmp/input.js"]) {
	test(`the filename ${filename} is rejected`, async () => {
		// Arrange
		using suite = register("tests/plugins/noDebugger.ts", {
			valid: [{ code: "", filename }],
			invalid: [],
		});
		const [testCase] = suite.cases;

		// Act
		const result = testCase.run();

		// Assert
		await assert.rejects(result, /filename/);
	});
}

test("settings other than the parser are rejected", async () => {
	// Arrange
	using suite = register("tests/plugins/noDebugger.ts", {
		valid: [
			{
				code: "",
				// @ts-expect-error Only parser settings are configurable.
				config: { linter: { enabled: false } },
			},
		],
		invalid: [],
	});
	const [testCase] = suite.cases;

	// Act
	const result = testCase.run();

	// Assert
	await assert.rejects(result, /unsupported field linter/);
});

test("concurrent cases do not interleave", async () => {
	// Arrange
	using suite = register("tests/plugins/stateful.ts", {
		valid: [],
		invalid: ["let x = 1;", "", "const y = 2;"].map((code) => ({
			code,
			diagnostics: [{ message: "Invocation 1" }],
		})),
	});

	// Act
	const result = Promise.all(suite.cases.map((testCase) => testCase.run()));

	// Assert
	await assert.doesNotReject(result);
});

test("a case cancelled by a write of another client reloads the plugin", async () => {
	// Arrange
	const count = 50;
	using stateful = register("tests/plugins/stateful.ts", {
		valid: [],
		invalid: Array.from({ length: count }, () => ({
			code: "let x = 1;",
			diagnostics: [{ message: "Invocation 1" }],
		})),
	});
	using writer = register("tests/plugins/noDebugger.ts", {
		valid: Array.from({ length: count }, () => "let x = 1;"),
		invalid: [],
	});

	// Act
	const result = Promise.all(
		[...stateful.cases, ...writer.cases].map((testCase) => testCase.run()),
	);

	// Assert
	await assert.doesNotReject(result);
});

test("package imports resolve on disk", async () => {
	// Arrange
	const directory = mkdtempSync(join(tmpdir(), "biome-plugin-imports-"));
	try {
		const dependency = join(directory, "node_modules", "fixture-rule");
		mkdirSync(dependency, { recursive: true });
		writeFileSync(
			join(dependency, "package.json"),
			JSON.stringify({
				name: "fixture-rule",
				type: "module",
				exports: "./index.js",
			}),
		);
		writeFileSync(
			join(dependency, "index.js"),
			`export { noDebugger } from ${JSON.stringify(resolve("tests/plugins/noDebugger.ts"))};`,
		);
		writeFileSync(
			join(directory, "plugin.js"),
			'export { noDebugger } from "fixture-rule";',
		);
		using suite = register(join(directory, "plugin.js"), {
			valid: [],
			invalid: [{ code: "debugger;", diagnostics: 1, output: "" }],
		});
		const [testCase] = suite.cases;

		// Act
		const result = testCase.run();

		// Assert
		await assert.doesNotReject(result);
	} finally {
		rmSync(directory, { recursive: true, force: true });
	}
});

test("an unreachable daemon fails the case", async () => {
	// Arrange
	using suite = register(
		"tests/plugins/noDebugger.ts",
		{ valid: ["let x = 1;"], invalid: [] },
		{ binaryPath: resolve("missing-engine") },
	);
	const [testCase] = suite.cases;

	// Act
	const result = testCase.run();

	// Assert
	await assert.rejects(result);
});

/** A stand-in client; the connection only forwards it to the request callback. */
interface FakeWorkspace {
	ping(): Promise<unknown>;
	destroy(): void;
}

const fake = (workspace: FakeWorkspace) => workspace as unknown as Workspace;
const ping = (workspace: Workspace) =>
	(workspace as unknown as FakeWorkspace).ping();

test("a timeout disposes the connection", async () => {
	// Arrange
	let destroyed = 0;
	const connection = createWorkspaceConnection(
		async () =>
			fake({
				ping: () => new Promise(() => {}),
				destroy: () => destroyed++,
			}),
		20,
	);

	// Act
	const first = connection.request(ping);
	await assert.rejects(first, /timed out/);
	const second = connection.request(ping);

	// Assert
	await assert.rejects(second, /disposed/);
	assert.equal(destroyed, 1);
});

test("an RPC error response keeps the connection", async () => {
	// Arrange
	let destroyed = 0;
	const connection = createWorkspaceConnection(async () =>
		fake({
			ping: () => Promise.reject({ code: -32603, message: "Internal error" }),
			destroy: () => destroyed++,
		}),
	);
	await assert.rejects(connection.request(ping), WorkspaceRpcError);

	// Act
	const result = connection.request(ping);

	// Assert
	await assert.rejects(result, WorkspaceRpcError);
	assert.equal(destroyed, 0);
});

test("a request cancelled for a concurrent write is left to the caller", async () => {
	// Arrange
	let attempts = 0;
	const connection = createWorkspaceConnection(async () =>
		fake({
			ping: async () => {
				attempts++;
				throw { code: -32801, message: "cancelled because of pending write" };
			},
			destroy() {},
		}),
	);

	// Act
	const result = connection.request(ping);

	// Assert
	await assert.rejects(
		result,
		(error) => error instanceof WorkspaceRpcError && error.code === -32801,
	);
	assert.equal(attempts, 1);
});

test("a connection arriving after a timeout is closed", async () => {
	// Arrange
	let finish: (workspace: Workspace) => void = () => {};
	let destroyed = 0;
	const connection = createWorkspaceConnection(
		() =>
			new Promise<Workspace>((resolve) => {
				finish = resolve;
			}),
		20,
	);
	await assert.rejects(connection.request(ping), /timed out/);

	// Act
	finish(
		fake({
			ping: () => assert.fail("must not execute"),
			destroy: () => destroyed++,
		}),
	);
	await new Promise((resolve) => setImmediate(resolve));

	// Assert
	assert.equal(destroyed, 1);
});

test("disposal rejects an outstanding request", async () => {
	// Arrange
	let entered: () => void = () => {};
	const ready = new Promise<void>((resolve) => {
		entered = resolve;
	});
	const connection = createWorkspaceConnection(async () =>
		fake({
			ping: () => {
				entered();
				return new Promise(() => {});
			},
			destroy() {},
		}),
	);
	const result = connection.request(ping);
	await ready;

	// Act
	connection.dispose();

	// Assert
	await assert.rejects(result, /disposed/);
});

test("the plugin API export is restricted to Biome", async () => {
	// Act
	const result = import("@biomejs/runtime/plugin");

	// Assert
	await assert.rejects(result, /intended to be used in Biome JS plugins/);
});

test("the transport-specific subpath is not public", async () => {
	// Act
	// @ts-expect-error The subpath is deliberately absent from the exports map.
	const result = import("@biomejs/runtime/plugin/testing/jsonrpc");

	// Assert
	await assert.rejects(result, { code: "ERR_PACKAGE_PATH_NOT_EXPORTED" });
});
