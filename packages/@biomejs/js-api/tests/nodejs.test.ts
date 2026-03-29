import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";
import { afterEach, beforeEach, describe, expect, it } from "vitest";
import { Biome, type ProjectKey } from "../dist/nodejs";

describe("Biome for Node.js", () => {
	let biome: Biome;
	let projectKey: ProjectKey;
	let projectDir: string;
	beforeEach(() => {
		projectDir = mkdtempSync(join(tmpdir(), "biome-js-api-"));
		biome = new Biome();
		const result = biome.openProject(projectDir);
		projectKey = result.projectKey;
	});

	afterEach(() => {
		biome.shutdown();
		rmSync(projectDir, { recursive: true, force: true });
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

	it("should load plugins from disk", () => {
		const pluginSource = [
			"`Object.assign($args)` where {",
			"\tregister_diagnostic(",
			"\t\tspan = $args,",
			'\t\tmessage = "Prefer object spread instead of `Object.assign()`"',
			"\t)",
			"}",
			"",
		].join("\n");

		writeFileSync(
			join(projectDir, "plugin.grit"),
			pluginSource,
		);

		biome.applyConfiguration(projectKey, {
			plugins: ["./plugin.grit"],
		});

		const result = biome.lintContent(
			projectKey,
			"const value = Object.assign({ foo: 'bar' });",
			{
				filePath: join(projectDir, "example.js"),
			},
		);

		expect(result.diagnostics).toHaveLength(1);
		expect(result.diagnostics[0].description).toEqual(
			"Prefer object spread instead of `Object.assign()`",
		);
	});
});
