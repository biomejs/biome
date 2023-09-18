import { afterEach, beforeEach, describe, expect, it } from "vitest";
import { Distribution, Biome } from "../dist";

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
});
