import { describe, expect, it } from "vitest";
import { spanInBytesToSpanInCodeUnits } from "../src/utils";

describe("spanInBytesToSpanInCodeUnits", () => {
	it("should return the same span for ASCII-only strings", () => {
		const str = "let a = 123";
		const result = spanInBytesToSpanInCodeUnits([0, 3], str);
		expect(result).toEqual([0, 3]);
		expect(str.slice(result[0], result[1])).toBe("let");
	});

	it("should convert byte offsets correctly for Chinese characters", () => {
		// Chinese characters are 3 bytes each in UTF-8
		// "/* 中文 */ let a = 123"
		// "/* " = 3 bytes (positions 0-2)
		// "中" = 3 bytes (positions 3-5)
		// "文" = 3 bytes (positions 6-8)
		// " */ " = 4 bytes (positions 9-12)
		// "let" = 3 bytes (positions 13-15)
		const str = "/* 中文 */ let a = 123";
		const result = spanInBytesToSpanInCodeUnits([13, 16], str);
		expect(str.slice(result[0], result[1])).toBe("let");
	});

	it("should handle spans at the start of a string with non-ASCII", () => {
		// "中" is 3 bytes in UTF-8 but 1 code unit in UTF-16
		const str = "中文abc";
		const result = spanInBytesToSpanInCodeUnits([0, 3], str);
		expect(str.slice(result[0], result[1])).toBe("中");
	});

	it("should handle multi-byte characters in the middle", () => {
		// "abc中def"
		// "abc" = 3 bytes (0-2)
		// "中" = 3 bytes (3-5)
		// "def" = 3 bytes (6-8)
		const str = "abc中def";
		const result = spanInBytesToSpanInCodeUnits([6, 9], str);
		expect(str.slice(result[0], result[1])).toBe("def");
	});

	it("should handle 2-byte UTF-8 characters", () => {
		// "é" is 2 bytes in UTF-8, 1 code unit in UTF-16
		// "café" = c(1) + a(1) + f(1) + é(2) = 5 bytes
		const str = "café!";
		const result = spanInBytesToSpanInCodeUnits([5, 6], str);
		expect(str.slice(result[0], result[1])).toBe("!");
	});

	it("should handle emoji (4-byte UTF-8 / surrogate pair)", () => {
		// "a😀b"
		// "a" = 1 byte (0)
		// "😀" = 4 bytes (1-4), 2 code units in UTF-16 (surrogate pair)
		// "b" = 1 byte (5)
		const str = "a😀b";
		const result = spanInBytesToSpanInCodeUnits([5, 6], str);
		expect(str.slice(result[0], result[1])).toBe("b");
	});

	it("should handle span covering an emoji", () => {
		// "a😀b"
		const str = "a😀b";
		const result = spanInBytesToSpanInCodeUnits([1, 5], str);
		expect(str.slice(result[0], result[1])).toBe("😀");
	});

	it("should handle empty span", () => {
		const str = "hello";
		const result = spanInBytesToSpanInCodeUnits([0, 0], str);
		expect(result).toEqual([0, 0]);
	});

	it("should handle full string span with non-ASCII", () => {
		const str = "中";
		const result = spanInBytesToSpanInCodeUnits([0, 3], str);
		expect(str.slice(result[0], result[1])).toBe("中");
	});
});
