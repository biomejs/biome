/* should generate diagnostics */

describe("one", () => {
	describe("one (nested)", () => {});
	describe("two (nested)", () => {});
	describe("three (nested)", () => {});
});

test.describe("two", () => {
	describe("one (nested)", () => {});
});

describe("three", () => {
	describe("one (nested)", () => {});
});

describe.each([1, 2])("four %d", () => {});

test.describe.serial("five", () => {});

describe.skipIf(isCI)("six", () => {});

function declareSuite() {
	describe("seven", () => {});
}
