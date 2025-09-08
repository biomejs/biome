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
});
