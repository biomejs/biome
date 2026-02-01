import { afterEach, beforeEach, describe, expect, it } from "vitest";
import type { ProjectKey } from "../../backend-jsonrpc/dist";
import { Biome, Distribution, spanInBytesToSpanInCodeUnits } from "../dist";

describe("spanInBytesToSpanInCodeUnits", () => {
	let biome: Biome;
	let projectKey: ProjectKey;

	beforeEach(async () => {
		biome = await Biome.create({
			distribution: Distribution.NODE,
		});
		const result = biome.openProject();
		projectKey = result.projectKey;
	});

	afterEach(() => {
		biome.shutdown();
	});

	it("should correctly convert byte span to code unit span for non-ASCII content", () => {
		// Content with non-ASCII characters (ç is 2 bytes in UTF-8)
		const content = "/** Franççççais */ let a = 123";
		const result = biome.lintContent(projectKey, content, {
			filePath: "example.js",
		});

		// Find the diagnostic for unused variable 'a'
		const diagnostic = result.diagnostics.find((d) =>
			d.description?.includes("unused"),
		);
		expect(diagnostic).toBeDefined();

		const [startBytes, endBytes] = diagnostic!.location.span;

		// Using byte offsets directly would give wrong result
		const incorrectSlice = content.slice(startBytes, endBytes);
		expect(incorrectSlice).not.toBe("a");

		// Using the conversion function gives correct result
		const [startCodeUnits, endCodeUnits] = spanInBytesToSpanInCodeUnits(
			diagnostic!.location.span,
			content,
		);
		const correctSlice = content.slice(startCodeUnits, endCodeUnits);
		expect(correctSlice).toBe("a");
	});

	it("should work correctly with ASCII-only content", () => {
		const content = "let unused = 123";
		const result = biome.lintContent(projectKey, content, {
			filePath: "example.js",
		});

		const diagnostic = result.diagnostics.find((d) =>
			d.description?.includes("unused"),
		);
		expect(diagnostic).toBeDefined();

		const [startCodeUnits, endCodeUnits] = spanInBytesToSpanInCodeUnits(
			diagnostic!.location.span,
			content,
		);
		const slice = content.slice(startCodeUnits, endCodeUnits);
		expect(slice).toBe("unused");
	});

	it("should handle emoji (surrogate pairs) correctly", () => {
		// Emoji are 4 bytes in UTF-8 and 2 code units in UTF-16
		const content = "/* 🎉 */ let a = 1";
		const result = biome.lintContent(projectKey, content, {
			filePath: "example.js",
		});

		const diagnostic = result.diagnostics.find((d) =>
			d.description?.includes("unused"),
		);
		expect(diagnostic).toBeDefined();

		const [startCodeUnits, endCodeUnits] = spanInBytesToSpanInCodeUnits(
			diagnostic!.location.span,
			content,
		);
		const slice = content.slice(startCodeUnits, endCodeUnits);
		expect(slice).toBe("a");
	});

	it("should handle mixed multi-byte characters", () => {
		// Mix of 1-byte, 2-byte, 3-byte, and 4-byte UTF-8 characters
		const content = "/* aéあ🎉 */ let x = 1";
		const result = biome.lintContent(projectKey, content, {
			filePath: "example.js",
		});

		const diagnostic = result.diagnostics.find((d) =>
			d.description?.includes("unused"),
		);
		expect(diagnostic).toBeDefined();

		const [startCodeUnits, endCodeUnits] = spanInBytesToSpanInCodeUnits(
			diagnostic!.location.span,
			content,
		);
		const slice = content.slice(startCodeUnits, endCodeUnits);
		expect(slice).toBe("x");
	});
});
