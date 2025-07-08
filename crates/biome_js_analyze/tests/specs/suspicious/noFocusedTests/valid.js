/* should not generate diagnostics */
describe.skip("test", () => {});
it.skip("test", () => {});
test.skip("test", () => {});
xdescribe("test", () => {});
xit("test", () => {})
describe.each([["a"], ["b"]])("%s", (a) => {});
it.each([["a"], ["b"]])("%s", (a) => {});
test.each([["a"], ["b"]])("%s", (a) => {});
foo.fit();