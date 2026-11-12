/* should generate diagnostics */

describe("sum", () => {
	beforeAll(() => {});
	beforeEach(() => {
		jest.useFakeTimers();
	});
	afterEach(() => {});
	afterAll(() => {});

	it("adds two numbers", () => {
		expect(1 + 2).toBe(3);
	});
	test.each([1, 2])("accepts %i", (value) => {});
});

fdescribe("focused", () => {
	fit("runs", () => {});
});

xdescribe("skipped", () => {
	xit("is skipped", () => {});
	xtest("is skipped", () => {});
});

// A use that isn't a call needs the global variable too.
const alias = describe;
const helpers = { expect };
