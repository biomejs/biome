import { afterEach, beforeEach, describe, expect, it } from "vitest";
import { Distribution, Rome } from "../dist";

describe("Rome WebAssembly DiagnosticPrinter", () => {
	let rome: Rome;
	beforeEach(async () => {
		rome = await Rome.create({
			distribution: Distribution.NODE,
		});
	});

	afterEach(() => {
		rome.shutdown();
	});

	it("should format content", () => {
		const SOURCE_CODE = `const variable = expr();

if(expr()) {
    statement();
}`;

		const html = rome.printDiagnostics(
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
					verbose_advices: {
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
					verbose_advices: {
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
