/* should generate diagnostics */
// withinDescribe="test": inside describe blocks must use test()

describe("suite", () => {
  it("foo", () => {});
  it.skip("foo", () => {});
  it.only("foo", () => {});
  xit("foo", () => {});
});
