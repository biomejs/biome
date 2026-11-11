/* should not generate diagnostics */

describe("one", () => {
	describe("one (nested)", () => {
		describe("one (nested twice)", () => {});
		describe("two (nested twice)", () => {});
	});
	describe("two (nested)", () => {});
	describe("three (nested)", () => {});

	it("my test", () => {});
});

// Not a describe block
test.describe.configure({ mode: "parallel" });
