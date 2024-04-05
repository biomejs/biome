import { afterEach, beforeEach, describe, expect, it } from "vitest";
import { Biome, Distribution } from "../dist";

describe("Biome WebAssembly lintContent", () => {
	const inputCode = `
	const a = "foo " + Date.now() + " bar";
	const b = /   /;
	`;

	let biome: Biome;
	beforeEach(async () => {
		biome = await Biome.create({
			distribution: Distribution.NODE,
		});
	});

	afterEach(() => {
		biome.shutdown();
	});

	describe("fixFileMode is undefined/omitted", () => {
		it("should emit diagnotics", () => {
			const result = biome.lintContent(inputCode, {
				filePath: "example.js",
			});
			expect(result.diagnostics).toMatchObject([
				{ category: "lint/style/useTemplate" },
				{
					category:
						"lint/complexity/noMultipleSpacesInRegularExpressionLiterals",
				},
			]);
		});
		it("should not fix the code", () => {
			const result = biome.lintContent(inputCode, {
				filePath: "example.js",
			});
			expect(result.content).toMatchSnapshot();
		});
	});

	describe("fixFileMode is SafeFixes", () => {
		it("should emit diagnotics", () => {
			const result = biome.lintContent(inputCode, {
				filePath: "example.js",
				fixFileMode: "SafeFixes",
			});
			expect(result.diagnostics).toMatchObject([
				{ category: "lint/style/useTemplate" },
			]);
		});
		it("should fix the SafeFixes only", () => {
			const result = biome.lintContent(inputCode, {
				filePath: "example.js",
				fixFileMode: "SafeFixes",
			});
			expect(result.content).toMatchSnapshot();
		});
	});

	describe("fixFileMode is SafeAndUnsafeFixes", () => {
		it("should emit diagnotics", () => {
			const result = biome.lintContent(inputCode, {
				filePath: "example.js",
				fixFileMode: "SafeAndUnsafeFixes",
			});
			expect(result.diagnostics).toHaveLength(0);
		});
		it("should fix the code", () => {
			const result = biome.lintContent(inputCode, {
				filePath: "example.js",
				fixFileMode: "SafeAndUnsafeFixes",
			});
			expect(result.content).toMatchSnapshot();
		});
	});
});
