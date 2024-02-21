describe("foo", () => {
	beforeEach(() => {});
	test("bar", () => {
		someFn();
	});
});

beforeEach(() => {});
test("bar", () => {
	someFn();
});

describe("foo", () => {
	beforeAll(() => {}), beforeEach(() => {});
	afterEach(() => {});
	afterAll(() => {});

	test("bar", () => {
		someFn();
	});
});

describe.skip("foo", () => {
	beforeEach(() => {}),
		beforeAll(() => {}),
		test("bar", () => {
			someFn();
		});
});
describe("foo", () => {
	beforeEach(() => {}),
		beforeAll(() => {}),
		test("bar", () => {
			someFn();
		});
});

describe("foo", () => {
	beforeEach(() => {}),
		test("bar", () => {
			someFn();
		});
	describe("inner_foo", () => {
		beforeEach(() => {});
		test("inner bar", () => {
			someFn();
		});
	});
});

describe.each(["hello"])("%s", () => {
	beforeEach(() => {});

	it("is fine", () => {});
});

describe("something", () => {
	describe("something", () => {
		beforeEach(() => {});

		it("is fine", () => {});
	});

	describe("something", () => {
		beforeEach(() => {});

		it("is fine", () => {});
	});
});
