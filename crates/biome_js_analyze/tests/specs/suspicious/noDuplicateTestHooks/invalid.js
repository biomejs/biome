describe("foo", () => {
	beforeEach(() => {
	});
	beforeEach(() => {
	});
	test("bar", () => {
		someFn();
	});
});

describe.skip("foo", () => {
	beforeEach(() => {
	});
	beforeAll(() => {
	});
	beforeAll(() => {
	});
	test("bar", () => {
		someFn();
	});
});

describe.skip("foo", () => {
	afterEach(() => {
	});
	afterEach(() => {
	});
	test("bar", () => {
		someFn();
	});
});

describe.skip("foo", () => {
	afterAll(() => {
	});
	afterAll(() => {
	});
	test("bar", () => {
		someFn();
	});
});

describe("foo", () => {
	beforeEach(() => {
	});
	beforeEach(() => {
	});
	beforeEach(() => {
	});
	test("bar", () => {
		someFn();
	});
});

describe.skip("foo", () => {
	afterAll(() => {
	});
	afterAll(() => {
	});
	beforeAll(() => {
	});
	beforeAll(() => {
	});
	test("bar", () => {
		someFn();
	});
});

describe("foo", () => {
	beforeEach(() => {
	});
	beforeEach(() => {
	});
	beforeAll(() => {
	});
	test("bar", () => {
		someFn();
	});
});

describe("foo", () => {
	beforeAll(() => {
	});
	test("bar", () => {
		someFn();
	});
	describe("inner_foo", () => {
		beforeEach(() => {
		});
		beforeEach(() => {
		});
		test("inner bar", () => {
			someFn();
		});
	});
});

describe.each(["hello"])("%s", () => {
	beforeEach(() => {
	});
	beforeEach(() => {
	});

	it("is not fine", () => {
	});
});

describe("something", () => {
	describe.each(["hello"])("%s", () => {
		beforeEach(() => {
		});

		it("is fine", () => {
		});
	});

	describe.each(["world"])("%s", () => {
		beforeEach(() => {
		});
		beforeEach(() => {
		});

		it("is not fine", () => {
		});
	});
});

describe("something", () => {
	describe.each(["hello"])("%s", () => {
		beforeEach(() => {
		});

		it("is fine", () => {
		});
	});

	describe.each(["world"])("%s", () => {
		describe("some more", () => {
			beforeEach(() => {
			});
			beforeEach(() => {
			});

			it("is not fine", () => {
			});
		});
	});
});

describe("foo", () => {
	before(() => {
	});
	before(() => {
	});
	test("bar", () => {
		someFn();
	});
});

describe("foo", () => {
	after(() => {
	});
	after(() => {
	});
	test("bar", () => {
		someFn();
	});
});
