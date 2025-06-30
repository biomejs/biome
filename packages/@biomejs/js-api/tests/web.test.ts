import { describe, expect, it } from "vitest";
import { Biome } from "../dist/web";

describe("Biome for web", () => {
	it("should export Biome", () => {
		expect(Biome).not.toBeUndefined();
	});
});
