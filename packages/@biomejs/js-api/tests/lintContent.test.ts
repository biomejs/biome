import { afterEach, beforeEach, describe, expect, it } from "vitest";
import { Distribution, Rome } from "../dist";

describe("Biome WebAssembly lintContent", () => {
	let biome: Rome;
	beforeEach(async () => {
		biome = await Rome.create({
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
