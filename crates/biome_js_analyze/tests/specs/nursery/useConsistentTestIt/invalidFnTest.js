/* should generate diagnostics */
// function="test": top-level tests must use test()

it("foo", () => {});
it.skip("foo", () => {});
it.only("foo", () => {});
xit("foo", () => {});
fit("foo", () => {});
