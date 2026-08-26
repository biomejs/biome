/* should not generate diagnostics */
// Default options: function="it", withinDescribe="it"

// it() is valid at top level
it("foo", () => {});
it.skip("foo", () => {});
it.only("foo", () => {});
it.concurrent("foo", () => {});
it.each([])(foo, () => {});
xit("foo", () => {});

// Non-test call expressions should not be flagged
notATest("foo", () => {});
myFunc("foo", () => {});

// it() inside describe is valid (default withinDescribe="it")
describe("suite", () => {
  it("foo", () => {});
  it.skip("foo", () => {});
  it.only("foo", () => {});
});

// Non-describe wrapper: test() here is still at top-level scope but the rule should not
// treat forEach as a describe block, meaning test() is still "top-level" and should use it()
// The following is correctly flagged (so we use it() here)
[1, 2, 3].forEach(() => {
  it("foo", () => {});
});

// When test functions are imported bindings the rule must not fire, because the
// safe rename fix cannot also update the import specifier (issue #11453).
import { test } from "vitest";
test("imported test should not be flagged", () => {});

import { it as myIt } from "@jest/globals";
myIt("aliased import should not be flagged", () => {});
