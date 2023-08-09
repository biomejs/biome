import { afterEach, beforeEach, describe, expect, it } from "vitest";
import { Distribution, Rome } from "../dist";

describe("Rome WebAssembly formatContent", () => {
	let rome: Rome;
	beforeEach(async () => {
		rome = await Rome.create({
			distribution: Distribution.NODE,
		});
	});

	afterEach(() => {
		rome.shutdown();
	});

	it("should format JavaScript content", () => {
		const result = rome.formatContent("function f   () {  }", {
			filePath: "example.js",
		});

		expect(result.content).toEqual("function f() {}\n");
		expect(result.diagnostics).toEqual([]);
	});

	it("should format JSON content", () => {
		const result = rome.formatContent(
			'{ "lorem": "ipsum", "foo": false, "bar": 23, "lorem": "ipsum", "foo": false, "bar": 23 }',
			{
				filePath: "example.json",
			},
		);

		expect(result.content).toEqual(
			'{\n\t"lorem": "ipsum",\n\t"foo": false,\n\t"bar": 23,\n\t"lorem": "ipsum",\n\t"foo": false,\n\t"bar": 23\n}\n',
		);
		expect(result.diagnostics).toEqual([]);
	});

	it("should not format and have diagnostics", () => {
		const content = "function   () {  }";
		const result = rome.formatContent(content, {
			filePath: "example.js",
		});

		expect(result.content).toEqual(content);
		expect(result.diagnostics).toHaveLength(1);
		expect(result.diagnostics[0].description).toContain(
			"expected a name for the function in a function declaration, but found none",
		);
		expect(result.diagnostics).toMatchSnapshot("syntax error");
	});

	it("should format content in debug mode", () => {
		const result = rome.formatContent("function f() {}", {
			filePath: "example.js",
			debug: true,
		});

		expect(result.content).toEqual("function f() {}\n");
		expect(result.diagnostics).toEqual([]);
		expect(result.ir).toMatchInlineSnapshot(
			'"[\\"function f\\", group([\\"()\\"]), \\" {}\\", hard_line_break]"',
		);
	});

	it("should not format content with range", () => {
		const result = rome.formatContent("let a   ; function g () {  }", {
			filePath: "file.js",
			range: [20, 25],
		});

		expect(result.content).toEqual("function g() {}");
		expect(result.diagnostics).toEqual([]);
	});

	it("should not format content with range in debug mode", () => {
		const result = rome.formatContent("let a   ; function g () {  }", {
			filePath: "file.js",
			range: [20, 25],
			debug: true,
		});

		expect(result.content).toEqual("function g() {}");
		expect(result.diagnostics).toEqual([]);
		expect(result.ir).toMatchInlineSnapshot(
			`
			"[
			  group([\\"let a\\"]),
			  \\";\\",
			  hard_line_break,
			  \\"function g\\",
			  group([\\"()\\"]),
			  \\" {}\\",
			  hard_line_break
			]"
		`,
		);
	});

	it("should format content with custom configuration (8 spaces, single quotes, preserve quotes)", () => {
		const content = `function   f() { return { "foo": 'bar' }  }`;
		const formatted = `function f() {
        return { 'foo': 'bar' };
}
`;

		rome.applyConfiguration({
			formatter: {
				indentStyle: "space",
				indentSize: 8,
			},
			javascript: {
				formatter: {
					quoteStyle: "single",
					quoteProperties: "preserve",
				},
			},
		});

		const result = rome.formatContent(content, {
			filePath: "example.js",
		});

		expect(result.content).toEqual(formatted);
	});

	it("should format content with custom configuration (8 spaces, jsx single quotes, preserve quotes)", () => {
		const content = `<div bar="foo" baz={"foo"} />`;
		const formatted = `<div bar='foo' baz={"foo"} />;
`;

		rome.applyConfiguration({
			formatter: {
				indentStyle: "space",
				indentSize: 8,
			},
			javascript: {
				formatter: {
					jsxQuoteStyle: "single",
					quoteProperties: "preserve",
				},
			},
		});

		const result = rome.formatContent(content, {
			filePath: "example.js",
		});

		expect(result.content).toEqual(formatted);
	});
});
