/* should generate diagnostics */
// Default options: function="it", withinDescribe="it"
// test() at top level should be renamed to it()

test("foo", () => {});
test.skip("foo", () => {});
test.only("foo", () => {});
test.concurrent("foo", () => {});
test.each([])(foo, () => {});
test.for([])("foo", () => {});
xtest("foo", () => {});
fit("foo", () => {});

// Inside describe, should also use it() (default withinDescribe="it")
describe("suite", () => {
  test("foo", () => {});
  test.skip("foo", () => {});
  test.only("foo", () => {});
  fit("foo", () => {});
});
