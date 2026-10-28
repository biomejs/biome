import assert from "node:assert/strict";
import { randomUUID } from "node:crypto";
import { tmpdir } from "node:os";
import { isAbsolute, join, normalize, resolve, sep } from "node:path";
import * as nodeTest from "node:test";
import { fileURLToPath } from "node:url";

import {
	createWorkspace,
	createWorkspaceWithBinary,
	type Diagnostic,
	type FixFileMode,
	type ProjectKey,
	type Severity,
	type Workspace,
} from "@biomejs/backend-jsonrpc";

import {
	createWorkspaceConnection,
	type WorkspaceConnection,
	WorkspaceRpcError,
} from "./jsonrpc.js";

type ErrorPhase = "load" | "parse";
type FixMode = "safe" | "unsafe";

interface TestConfiguration {
	javascript?: {
		parser?: {
			jsxEverywhere?: boolean;
			unsafeParameterDecoratorsEnabled?: boolean;
		};
	};
}

interface Position {
	line: number;
	column: number;
}

interface TestDiagnostic {
	message: string;
	severity: Severity;
	start?: Position;
	end?: Position;
}

interface ExpectedDiagnostic {
	message?: string | RegExp;
	severity?: Severity;
	start?: Position;
	end?: Position;
}

export interface ValidCase {
	code: string;
	name?: string;
	filename?: string;
	config?: TestConfiguration;
}

export interface InvalidCase extends ValidCase {
	diagnostics: number | readonly ExpectedDiagnostic[];
	output?: string | null;
	fixMode?: FixMode;
}

export interface RuleTestSuite {
	valid: readonly (string | ValidCase)[];
	invalid: readonly InvalidCase[];
}

/** Registers a group of tests, like `describe` in Vitest, Jest, Mocha or `node:test`. */
export type DescribeHook = (name: string, fn: () => void) => unknown;

/** Registers a test, like `it`. */
export type ItHook = (name: string, fn: () => Promise<void>) => unknown;

/** Registers a hook that runs after the tests of the enclosing group, like `afterAll`. */
export type AfterAllHook = (fn: () => void) => unknown;

export interface RuleTesterOptions {
	binaryPath?: string;
	timeout?: number;
	filename?: string;
	config?: TestConfiguration;
}

const MAX_TITLE_LENGTH = 80;
const SOURCE_EXTENSION = /\.(?:js|jsx|mjs|cjs|ts|tsx|mts|cts)$/i;
const FIX_FILE_MODES: Record<FixMode, FixFileMode> = {
	safe: "safeFixes",
	unsafe: "safeAndUnsafeFixes",
};

export class RuleExecutionError extends Error {
	readonly phase: ErrorPhase;

	constructor(phase: ErrorPhase, message: string) {
		super(`Biome ${phase} failure: ${message}`);

		this.name = "RuleExecutionError";
		this.phase = phase;
	}
}

/**
 * Asserts that `value` is a plain object whose keys are all in `allowed`.
 *
 * Callers may be untyped JavaScript, so every public input is checked at runtime.
 */
function assertFields(
	value: unknown,
	allowed: readonly string[],
	label: string,
): asserts value is object {
	assert(
		typeof value === "object" && value !== null && !Array.isArray(value),
		`${label} must be an object`,
	);
	for (const key of Object.keys(value)) {
		assert(allowed.includes(key), `${label}: unsupported field ${key}`);
	}
}

function validateConfiguration(
	config: TestConfiguration = {},
): TestConfiguration {
	assertFields(config, ["javascript"], "config");

	const { javascript } = config;
	if (javascript === undefined) {
		return config;
	}

	assertFields(javascript, ["parser"], "config.javascript");

	const { parser } = javascript;
	if (parser === undefined) {
		return config;
	}

	assertFields(
		parser,
		["jsxEverywhere", "unsafeParameterDecoratorsEnabled"],
		"config.javascript.parser",
	);

	for (const option of Object.values(parser)) {
		assert.equal(typeof option, "boolean", "parser options must be booleans");
	}

	return config;
}

/** Resolves a relative path against the current working directory. */
function resolvePluginPath(plugin: string | URL): string {
	if (plugin instanceof URL) {
		return fileURLToPath(plugin);
	}

	assert.equal(typeof plugin, "string", "plugin must be a path or file URL");

	return resolve(plugin);
}

function validateFilename(filename: unknown): string {
	assert(
		typeof filename === "string" && SOURCE_EXTENSION.test(filename),
		"filename must select a JS/TS source",
	);

	const normalized = normalize(filename);

	assert(
		!isAbsolute(normalized) &&
			normalized !== ".." &&
			!normalized.startsWith(`..${sep}`),
		"filename must be relative to the project root",
	);

	return normalized;
}

function validateInvalidCase(testCase: InvalidCase): void {
	assert(
		testCase.diagnostics !== undefined,
		"invalid cases require diagnostics",
	);
	assert(
		testCase.output === undefined ||
			testCase.output === null ||
			typeof testCase.output === "string",
		"output must be a string or null",
	);
	assert(
		testCase.fixMode === undefined ||
			Object.hasOwn(FIX_FILE_MODES, testCase.fixMode),
		"fixMode must be safe or unsafe",
	);
	assert(
		testCase.fixMode === undefined || testCase.output !== undefined,
		"fixMode requires output",
	);
}

function position(code: Buffer, offset: number): Position {
	const lines = code
		.subarray(0, offset)
		.toString()
		.split(/\r\n|\r|\n/);

	return {
		line: lines.length,
		column: [...(lines.at(-1) ?? "")].length + 1,
	};
}

function toTestDiagnostic(
	diagnostic: Diagnostic,
	code: Buffer,
): TestDiagnostic {
	const result: TestDiagnostic = {
		message: diagnostic.description,
		severity: diagnostic.severity,
	};

	const span = diagnostic.location.span;
	if (span) {
		result.start = position(code, span[0]);
		result.end = position(code, span[1]);
	}

	return result;
}

function checkDiagnostics(
	actual: readonly TestDiagnostic[],
	expected: InvalidCase["diagnostics"],
): void {
	if (typeof expected === "number") {
		assert(
			Number.isSafeInteger(expected) && expected > 0,
			"diagnostics must be a positive integer",
		);
		assert.equal(actual.length, expected, "diagnostic count");

		return;
	}

	assert(
		Array.isArray(expected) && expected.length > 0,
		"invalid cases require nonempty diagnostics",
	);
	assert.equal(actual.length, expected.length, "diagnostic count");

	for (const [index, expectation] of expected.entries()) {
		assertFields(
			expectation,
			["message", "severity", "start", "end"],
			`diagnostics[${index}]`,
		);
		assert(
			Object.keys(expectation).length > 0,
			"use a count instead of an empty diagnostic expectation",
		);

		const diagnostic = actual[index] as unknown as Record<string, unknown>;
		for (const [key, value] of Object.entries(expectation)) {
			const label = `diagnostic ${index}: ${key}`;
			if (key === "message" && value instanceof RegExp) {
				assert.match(diagnostic.message as string, value, label);
			} else {
				assert.deepEqual(diagnostic[key], value, label);
			}
		}
	}
}

function caseTitle(testCase: unknown): string {
	const { name, code } =
		typeof testCase === "string"
			? { name: undefined, code: testCase }
			: ((testCase ?? {}) as { name?: unknown; code?: unknown });

	const title = String(name ?? code);
	if (title.length === 0) {
		return "(empty)";
	}

	return title.length > MAX_TITLE_LENGTH
		? `${title.slice(0, MAX_TITLE_LENGTH - 1)}…`
		: title;
}

/** Looks up a test runner function: the hook if set, then a global, then `node:test`. */
function resolveHook<T>(
	hook: T | undefined,
	globals: string[],
	fallback: T,
): T {
	if (hook !== undefined) {
		return hook;
	}

	const scope = globalThis as Record<string, unknown>;
	const global = globals.map((name) => scope[name]).find(Boolean);

	return (global as T | undefined) ?? fallback;
}

/**
 * Runs the cases of one suite: a Workspace connection and a virtual project
 * with only the plugin under test enabled.
 *
 * Every case reloads the plugin, so module state does not leak between cases.
 */
class Session {
	readonly #connection: WorkspaceConnection;
	readonly #plugin: string;
	readonly #filename: string;
	readonly #config: TestConfiguration;
	/** A virtual root, unique per session, so plugin settings never collide. */
	readonly #root = join(tmpdir(), `biome-rule-tester-${randomUUID()}`);
	#project: Promise<ProjectKey> | undefined;
	/** Cases share one project, so they must not interleave. */
	#queue: Promise<unknown> = Promise.resolve();

	constructor(
		connection: WorkspaceConnection,
		plugin: string,
		filename: string,
		config: TestConfiguration,
	) {
		this.#connection = connection;
		this.#plugin = plugin;
		this.#filename = filename;
		this.#config = config;
	}

	dispose(): void {
		this.#connection.dispose();
	}

	async assertValid(testCase: string | ValidCase): Promise<void> {
		const validCase =
			typeof testCase === "string" ? { code: testCase } : testCase;

		assertFields(validCase, ["name", "code", "filename", "config"], "case");

		const { diagnostics } = await this.#execute(validCase, undefined);

		assert.deepEqual(diagnostics, [], "valid case reported diagnostics");
	}

	async assertInvalid(testCase: InvalidCase): Promise<void> {
		assertFields(
			testCase,
			[
				"name",
				"code",
				"filename",
				"config",
				"diagnostics",
				"output",
				"fixMode",
			],
			"case",
		);
		validateInvalidCase(testCase);

		const fixMode =
			testCase.output === undefined ? undefined : (testCase.fixMode ?? "safe");

		const { diagnostics, output } = await this.#execute(testCase, fixMode);

		checkDiagnostics(diagnostics, testCase.diagnostics);

		if (fixMode !== undefined) {
			assert.equal(output, testCase.output ?? testCase.code, "fix output");
		}
	}

	#call<T>(fn: (workspace: Workspace) => Promise<T>): Promise<T> {
		return this.#connection.request(fn);
	}

	#projectKey(): Promise<ProjectKey> {
		this.#project ??= this.#call((workspace) =>
			workspace.openProject({ path: this.#root, openUninitialized: true }),
		).then(
			({ projectKey }) => projectKey,
			(error) => {
				this.#project = undefined;
				throw error;
			},
		);

		return this.#project;
	}

	/** Enables only the plugin under test, which also reloads it with fresh module state. */
	async #loadSettings(config: TestConfiguration): Promise<ProjectKey> {
		const projectKey = await this.#projectKey();

		try {
			await this.#call((workspace) =>
				workspace.updateSettings({
					projectKey,
					workspaceDirectory: this.#root,
					configuration: {
						plugins: [this.#plugin],
						linter: { enabled: true, rules: { recommended: false } },
						assist: { enabled: false },
						formatter: { enabled: false },
						// Always sent: an omitted key would keep the previous case's settings.
						javascript: config.javascript ?? {},
					},
				}),
			);
		} catch (error) {
			if (error instanceof WorkspaceRpcError) {
				throw new RuleExecutionError("load", error.message);
			}

			throw error;
		}

		return projectKey;
	}

	/** Lints `testCase` with only the plugin under test enabled, then applies fixes if requested. */
	#execute(
		testCase: ValidCase,
		fixMode: FixMode | undefined,
	): Promise<{ diagnostics: TestDiagnostic[]; output?: string }> {
		assert.equal(typeof testCase.code, "string", "code must be a string");

		const config = validateConfiguration(testCase.config ?? this.#config);
		const path = join(
			this.#root,
			validateFilename(testCase.filename ?? this.#filename),
		);

		return this.#serial(async () => {
			const projectKey = await this.#loadSettings(config);

			await this.#call((workspace) =>
				workspace.openFile({
					projectKey,
					path,
					content: { type: "fromClient", content: testCase.code, version: 0 },
				}),
			);

			try {
				const result = await this.#call((workspace) =>
					workspace.pullDiagnostics({
						projectKey,
						path,
						categories: ["syntax", "lint"],
					}),
				);

				const isPluginDiagnostic = (diagnostic: Diagnostic) =>
					diagnostic.category === "plugin";

				if (result.parseErrors > 0) {
					throw new RuleExecutionError(
						"parse",
						result.diagnostics
							.filter((diagnostic) => !isPluginDiagnostic(diagnostic))
							.map((diagnostic) => diagnostic.description)
							.join("\n"),
					);
				}

				const code = Buffer.from(testCase.code);
				const diagnostics = result.diagnostics
					.filter(isPluginDiagnostic)
					.map((diagnostic) => toTestDiagnostic(diagnostic, code));

				if (fixMode === undefined) {
					return { diagnostics };
				}

				const { code: output } = await this.#call((workspace) =>
					workspace.fixFile({
						projectKey,
						path,
						fixFileMode: FIX_FILE_MODES[fixMode],
						shouldFormat: false,
						ruleCategories: ["lint"],
					}),
				);

				return { diagnostics, output };
			} finally {
				// Do not mask the original failure, e.g. a timeout that disposed the connection.
				await this.#call((workspace) =>
					workspace.closeFile({ projectKey, path }),
				).catch(() => {});
			}
		});
	}

	#serial<T>(task: () => Promise<T>): Promise<T> {
		const result = this.#queue.then(task);
		this.#queue = result.catch(() => {});

		return result;
	}
}

/**
 * Registers the cases of a plugin module as tests of the current test runner,
 * and runs them through the public Workspace API of a Biome daemon.
 *
 * The runner is taken from {@link RuleTester.describe}, {@link RuleTester.it} and
 * {@link RuleTester.afterAll} if set, then from the globals of Vitest, Jest or
 * Mocha, and finally from `node:test`.
 */
export class RuleTester {
	static describe: DescribeHook | undefined;
	static it: ItHook | undefined;
	static afterAll: AfterAllHook | undefined;

	readonly #options: RuleTesterOptions;
	readonly #filename: string;
	readonly #config: TestConfiguration;

	constructor(options: RuleTesterOptions = {}) {
		assertFields(
			options,
			["binaryPath", "timeout", "filename", "config"],
			"RuleTester options",
		);

		this.#options = options;
		this.#filename = validateFilename(options.filename ?? "input.js");
		this.#config = validateConfiguration(options.config);
	}

	/**
	 * Registers a group named `name` with one test per case. `plugin` is a file
	 * URL, or a path relative to the current working directory. All rules the
	 * module exports run, so test one rule per module.
	 */
	run(name: string, plugin: string | URL, cases: RuleTestSuite): void {
		const pluginPath = resolvePluginPath(plugin);

		assertFields(cases, ["valid", "invalid"], "cases");
		assert(
			Array.isArray(cases.valid) && Array.isArray(cases.invalid),
			"valid and invalid must be arrays",
		);
		assert(
			cases.valid.length + cases.invalid.length > 0,
			"suite must contain a case",
		);

		const describe = resolveHook(
			RuleTester.describe,
			["describe"],
			nodeTest.describe as DescribeHook,
		);
		const it = resolveHook(RuleTester.it, ["it"], nodeTest.it as ItHook);
		const afterAll = resolveHook(
			RuleTester.afterAll,
			["afterAll", "after"],
			nodeTest.after as AfterAllHook,
		);

		// `BIOME_BINARY` is also honored by the `biome` command of `@biomejs/biome`.
		const { binaryPath = process.env.BIOME_BINARY, timeout } = this.#options;
		const session = new Session(
			createWorkspaceConnection(
				binaryPath === undefined
					? createWorkspace
					: () => createWorkspaceWithBinary(binaryPath),
				timeout,
			),
			pluginPath,
			this.#filename,
			this.#config,
		);

		describe(name, () => {
			afterAll(() => session.dispose());

			// Runners may report a group without tests as an error.
			if (cases.valid.length > 0) {
				describe("valid", () => {
					for (const testCase of cases.valid) {
						it(caseTitle(testCase), () => session.assertValid(testCase));
					}
				});
			}

			if (cases.invalid.length > 0) {
				describe("invalid", () => {
					for (const testCase of cases.invalid) {
						it(caseTitle(testCase), () => session.assertInvalid(testCase));
					}
				});
			}
		});
	}
}
