/* should not generate diagnostics */
// withinDescribe="test": using test() inside describe is valid

describe("suite", () => {
  test("foo", () => {});
  test.skip("foo", () => {});
  test.only("foo", () => {});
  xtest("foo", () => {});
});

// Outside describe: it() is valid (function="it" default)
it("top-level", () => {});
it.skip("top-level", () => {});
