import { afterEach, beforeEach, describe, expect, it } from "vitest";
import type { ProjectKey } from "../../backend-jsonrpc/dist";
import { Biome, Distribution } from "../dist";

describe("Biome WebAssembly lintContent", () => {
	const inputCode = `
	debugger;
	const b = /   /;
	`;

	let biome: Biome;
	let projectKey: ProjectKey;
	beforeEach(async () => {
		biome = await Biome.create({
			distribution: Distribution.NODE,
		});
		projectKey = biome.openProject();
	});

	afterEach(() => {
		biome.shutdown();
	});

	describe("fixFileMode is undefined/omitted", () => {
		it("should emit diagnotics", () => {
			const result = biome.lintContent(projectKey, inputCode, {
				filePath: "example.js",
			});
			const categories = result.diagnostics.map((d) => d.category);
			expect(categories).toMatchObject([
				"lint/suspicious/noDebugger",
				"lint/complexity/noAdjacentSpacesInRegex",
				"lint/correctness/noUnusedVariables",
			]);
		});
		it("should not fix the code", () => {
			const result = biome.lintContent(projectKey, inputCode, {
				filePath: "example.js",
			});
			expect(result.content).toMatchSnapshot();
		});
	});

	describe("fixFileMode is SafeFixes", () => {
		it("should emit diagnotics", () => {
			const result = biome.lintContent(projectKey, inputCode, {
				filePath: "example.js",
				fixFileMode: "safeFixes",
			});
			const categories = result.diagnostics.map((d) => d.category);
			expect(categories).toMatchObject([
				"lint/suspicious/noDebugger",
				"lint/correctness/noUnusedVariables",
			]);
		});
		it("should fix the SafeFixes only", () => {
			const result = biome.lintContent(projectKey, inputCode, {
				filePath: "example.js",
				fixFileMode: "safeFixes",
			});
			expect(result.content).toMatchSnapshot();
		});
	});

	describe("fixFileMode is SafeAndUnsafeFixes", () => {
		it("should emit diagnotics", () => {
			const result = biome.lintContent(projectKey, inputCode, {
				filePath: "example.js",
				fixFileMode: "safeAndUnsafeFixes",
			});
			expect(result.diagnostics).toHaveLength(0);
		});
		it("should fix the code", () => {
			const result = biome.lintContent(projectKey, inputCode, {
				filePath: "example.js",
				fixFileMode: "safeAndUnsafeFixes",
			});
			expect(result.content).toMatchSnapshot();
		});
	});
});
