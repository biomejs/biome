import { afterEach, beforeEach, describe, expect, it } from "vitest";
import { Biome, type ProjectKey } from "../dist/nodejs";

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

	it("should parse GritQL patterns", () => {
		const pattern = "`const $x = 1;`";

		const patternId = biome.parsePattern(pattern, "JavaScript");
		expect(patternId).toBeDefined();
		expect(typeof patternId).toBe("string");

		biome.dropPattern(patternId);
	});

	it("should search GritQL patterns", () => {
		const code = "const x = 1; const y = 2;";
		const pattern = "`const $x = 1;`";

		const patternId = biome.parsePattern(pattern, "JavaScript");

		const matches = biome.searchPattern(projectKey, "test.js", code, patternId);
		expect(matches).toBeDefined();
		expect(Array.isArray(matches)).toBe(true);

		biome.dropPattern(patternId);
	});

	it("should search CSS GritQL patterns", () => {
		const code = "div { color: green; }";
		const pattern = "`color: $x`";

		const patternId = biome.parsePattern(pattern, "CSS");

		const matches = biome.searchPattern(
			projectKey,
			"test.css",
			code,
			patternId,
		);
		expect(matches).toBeDefined();
		expect(Array.isArray(matches)).toBe(true);
		expect(matches.length).toBeGreaterThan(0);

		biome.dropPattern(patternId);
	});
});
