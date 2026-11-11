/* should generate diagnostics */

beforeAll(() => {});
beforeEach(() => {});

test("above a describe block", () => {});

describe("test suite", () => {
	it("my test", () => {});
});

it("below a describe block", () => {});

afterEach(() => {});
afterAll(() => {});

// Modifiers
it.skip("skipped", () => {});
it.only("focused", () => {});
it.todo("to do");
test.concurrent("concurrent", async () => {});
test.fixme("broken", async ({ page }) => {});
test.skip("skipped with details", { tag: "@slow" }, async ({ page }) => {});
xit("skipped", () => {});
xtest("skipped", () => {});
fit("focused", () => {});

// Tables
it.each([1, 2, 3])("%n", () => {});
it.skip.each([1, 2, 3])("%n", () => {});
it.each`
	value
	${1}
`("$value", () => {});
it.skip.each`
	value
	${1}
`("$value", () => {});
test.for([1, 2, 3])("%n", () => {});

// Conditions
test.skipIf(isCI)("skipped in CI", () => {});
test.runIf(isCI)("only in CI", () => {});

// Playwright
test.beforeAll(async () => {});
test.beforeEach(async ({ page }) => {});
test.afterEach(async ({ page }) => {});
test.afterAll(async () => {});

// Mocha and Node.js
before(() => {});
after(() => {});

// Not directly at the top level of the file, but still outside a describe block
function declareTests() {
	test("in a function", () => {});
}

if (isCI) {
	test("in a condition", () => {});
}

beforeEach(() => {
	describe("in a hook", () => {
		test("is inside a describe block", () => {});
	});
});

// The describe block is an argument, so it doesn't contain the test.
test("my test", describe("test suite", () => {}));
