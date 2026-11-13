/* should generate diagnostics */
beforeAll(() => {});
beforeEach(() => {});
afterEach(() => {});
afterAll(() => {});
before(() => {});
after(() => {});

describe("foo", () => {
	let foo;

	beforeEach(() => {
		foo = setupFoo();
	});

	afterEach(() => {
		foo = null;
	});

	it("does something", () => {
		expect(foo.doesSomething()).toBe(true);
	});

	describe("with bar", () => {
		let bar;

		beforeEach(() => {
			bar = setupBar();
		});

		afterEach(() => {
			bar = null;
		});

		it("does something with bar", () => {
			expect(foo.doesSomething(bar)).toBe(true);
		});
	});
});

test.beforeAll(async () => {});
test.beforeEach(async ({ page }) => {});
test.afterEach(async () => {});
test.afterAll(async () => {});
it.beforeEach(() => {});
describe.afterAll(() => {});
suite.beforeEach(() => {});

(beforeEach)(() => {});
beforeEach(setup);
afterAll(async function () {}, 1000);
