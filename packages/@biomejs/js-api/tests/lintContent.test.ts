import { afterEach, beforeEach, describe, expect, it } from "vitest";
import { Biome, Distribution } from "../dist";

describe("Biome WebAssembly lintContent", () => {
	let biome: Biome;
	beforeEach(async () => {
		biome = await Biome.create({
			distribution: Distribution.NODE,
		});
	});

	afterEach(() => {
		biome.shutdown();
	});

	it("should lint content", () => {
		const result = biome.lintContent("if (a == b) {}", {
			filePath: "example.js",
		});

		expect(result.diagnostics).toMatchSnapshot("lint diagnostics");
	});

	it("should not fix lint issues if fixFileMode is undefined/omitted", () => {
		const result = biome.lintContent("let a = \"foo \" + Date.now() + \" bar\"", {
			filePath: "example.js",
		});

		expect(result.content).toMatchSnapshot("original content");
	})

	it("should fix lint issues and return new content if fileFileMode is SafeAndUnsafeFixes", () => {
		const result = biome.lintContent("let a = \"foo \" + Date.now() + \" bar\"", {
			filePath: "example.js",
			fixFileMode: "SafeAndUnsafeFixes",
		});

		expect(result.content).toMatchSnapshot("fixed content");
	})
});
