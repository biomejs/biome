import { describe, expect, test } from "bun:test";

describe("math", () => {
	test("adds", () => {
		expect(1 + 1).toBe(2);
	});
});

test("top level", () => {
	expect(true).toBe(true);
});
