import { afterEach, beforeEach, describe, expect, it } from "vitest";
import { Distribution, Rome } from "../dist";

describe("Rome WebAssembly lintContent", () => {
	let rome: Rome;
	beforeEach(async () => {
		rome = await Rome.create({
			distribution: Distribution.NODE,
		});
	});

	afterEach(() => {
		rome.shutdown();
	});

	it("should lint content", () => {
		const result = rome.lintContent("if (a == b) {}", {
			filePath: "example.js",
		});

		expect(result.diagnostics).toMatchSnapshot("lint diagnostics");
	});
});
