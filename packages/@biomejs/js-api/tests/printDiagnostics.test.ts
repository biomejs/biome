import { afterEach, beforeEach, describe, expect, it } from "vitest";
import { Biome, Distribution } from "../dist";

describe("Biome WebAssembly DiagnosticPrinter", () => {
	let biome: Biome;
	beforeEach(async () => {
		biome = await Biome.create({
			distribution: Distribution.NODE,
		});
	});

	afterEach(() => {
		biome.shutdown();
	});

	it("should format content", () => {
		const SOURCE_CODE = `const variable = expr();

if(expr()) {
    statement();
}`;

		const html = biome.printDiagnostics(
			[
				{
					advices: {
						advices: [],
					},
					category: "parse",
					description: "error description content",
					location: {
						path: {
							file: "file.js",
						},
						span: [31, 37],
					},
					message: [
						{
							content: "error message content",
							elements: [],
						},
					],
					severity: "error",
					tags: [],
					verboseAdvices: {
						advices: [],
					},
				},
				{
					advices: {
						advices: [],
					},
					category: "parse",
					description: "error description content",
					location: {
						path: {
							file: "file.js",
						},
						span: [46, 58],
					},
					message: [
						{
							content: "error message content",
							elements: [],
						},
					],
					severity: "error",
					tags: [],
					verboseAdvices: {
						advices: [],
					},
				},
			],
			{
				filePath: "file.js",
				fileSource: SOURCE_CODE,
				verbose: true,
			},
		);

		expect(html).toMatchSnapshot("HTML diagnostic");
	});
});
