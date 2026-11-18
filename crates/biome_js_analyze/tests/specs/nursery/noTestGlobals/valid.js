/* should not generate diagnostics */
import { describe, expect, it as itChecks, jest } from "@jest/globals";
const { beforeEach } = require("@jest/globals");
const { afterEach } = require("my-test-library");

describe("sum", () => {
	beforeEach(() => {});
	afterEach(() => {});
	itChecks("adds two numbers", () => {
		expect(jest.fn()).not.toHaveBeenCalled();
	});
});

// Declared in the file.
function test(name) {}
test("sum");
function run(xit) {
	xit("is a parameter");
}

// `typeof` accepts a name that doesn't exist.
if (typeof afterAll === "function") {
}
if (typeof (fit) !== "undefined") {
}

// Not a reference to a global variable.
runner.fdescribe("sum");
const options = { xtest: true };

// Without a dependency on Vitest, the global variables of Vitest aren't checked.
vi.useFakeTimers();
suite("sum", () => {});
assert.equal(1 + 2, 3);
