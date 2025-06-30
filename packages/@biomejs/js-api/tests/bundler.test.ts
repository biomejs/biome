import { describe, expect, it } from "vitest";
import { Biome } from "../dist/bundler";

describe("Biome for bundler", () => {
	it("should export Biome", () => {
		expect(Biome).not.toBeUndefined();
	});
});
