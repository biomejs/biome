/* should generate diagnostics */
// Test non-function callbacks
test.describe("foo", "foo2");

test.describe("bar", 42);

test.describe("baz", { tag: ["@slow"] });

describe("qux", null);

describe("suite", undefined);

test.describe("another", someVariable);
