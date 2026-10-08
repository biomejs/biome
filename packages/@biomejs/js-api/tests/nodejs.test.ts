import { afterEach, beforeEach, describe, expect, it } from "vitest";
import {
	Biome,
	type ProjectKey,
	spanInBytesToSpanInCodeUnits,
} from "../dist/nodejs";

describe("Biome for Node.js", () => {
	let biome: Biome;
	let projectKey: ProjectKey;
	beforeEach(() => {
		biome = new Biome();
		const result = biome.openProject();
		projectKey = result.projectKey;
	});

	afterEach(() => {
		biome.shutdown();
	});

	it("should format content", () => {
		const result = biome.formatContent(projectKey, "let foo  = 'bar'", {
			filePath: "example.js",
		});

		expect(result.content).toEqual('let foo = "bar";\n');
		expect(result.diagnostics).toEqual([]);
	});

	it("should emit diagnostics", () => {
		const result = biome.lintContent(projectKey, "a { font-color: red }", {
			filePath: "example.css",
		});
		expect(result.diagnostics).toHaveLength(1);
		expect(result.diagnostics[0].description).toEqual(
			"Unknown property is not allowed.",
		);
	});

	it("should search content with a GritQL pattern", () => {
		const patternId = biome.parsePattern("`const $x = 1;`", {
			defaultLanguage: "js",
		});

		try {
			const result = biome.searchContent(
				projectKey,
				"const x = 1; const y = 2;",
				{ filePath: "example.js", patternId },
			);
			expect(result.matches).toEqual([[0, 12]]);
		} finally {
			biome.dropPattern(patternId);
		}
	});

	it("should search CSS content with a GritQL pattern", () => {
		const patternId = biome.parsePattern("`color: $x`", {
			defaultLanguage: "css",
		});

		try {
			const result = biome.searchContent(projectKey, "div { color: green; }", {
				filePath: "example.css",
				patternId,
			});
			expect(result.matches).toEqual([[6, 18]]);
		} finally {
			biome.dropPattern(patternId);
		}
	});

	it("should return GritQL matches as byte offsets", () => {
		const content = 'const a = "é"; const b = 1;';
		const patternId = biome.parsePattern("`const $x = 1;`", {
			defaultLanguage: "js",
		});

		try {
			const { matches } = biome.searchContent(projectKey, content, {
				filePath: "example.js",
				patternId,
			});
			expect(matches).toEqual([[16, 28]]);

			const [start, end] = spanInBytesToSpanInCodeUnits(matches[0], content);
			expect(content.slice(start, end)).toEqual("const b = 1;");
		} finally {
			biome.dropPattern(patternId);
		}
	});

	it("should throw when searching with a dropped GritQL pattern", () => {
		const patternId = biome.parsePattern("`const $x = 1;`", {
			defaultLanguage: "js",
		});
		biome.dropPattern(patternId);

		expect(() =>
			biome.searchContent(projectKey, "const x = 1;", {
				filePath: "example.js",
				patternId,
			}),
		).toThrow();
	});

	it("should throw when parsing an invalid GritQL pattern", () => {
		expect(() =>
			biome.parsePattern("`const $x = 1;", { defaultLanguage: "js" }),
		).toThrow();
	});
});
