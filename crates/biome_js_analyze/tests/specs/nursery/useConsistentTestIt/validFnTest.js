/* should not generate diagnostics */
// function="test": top-level tests using test() are valid

test("foo", () => {});
test.skip("foo", () => {});
test.only("foo", () => {});
test.concurrent("foo", () => {});
test.each([])(foo, () => {});
xtest("foo", () => {});

// Non-test calls should not be flagged
notATest("foo", () => {});

// When `it` is imported the rule must not fire, because the safe rename fix
// would only update the call site and leave the import specifier broken.
import { it } from "vitest";
it("imported it should not be flagged", () => {});
