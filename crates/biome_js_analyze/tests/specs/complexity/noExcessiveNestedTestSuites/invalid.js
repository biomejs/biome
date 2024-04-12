describe("foo", function () {
	describe("bar", function () {
		describe("baz", function () {
			describe("qux", function () {
				describe("quxx", function () {
					describe("over limit", function () {
						describe("qux nested", function () {
                            it("should get something", () => {
                                expect(getSomething()).toBe("Something");
                            });
                        });
					});
				});
			});
		});
	});
});

describe("foo", () => {
	describe("bar", () => {
		describe("baz", () => {
			describe("baz1", () => {
				describe("baz2", () => {
					describe("baz3", () => {
						it("should get something", () => {
							expect(getSomething()).toBe("Something");
						});
					});

					describe("baz4", () => {
						it("should get something", () => {
							expect(getSomething()).toBe("Something");
						});
					});
				});
			});
		});

		describe("qux", function () {
			it("should get something", () => {
				expect(getSomething()).toBe("Something");
			});
		});
	});
});
