/* should generate diagnostics */
// Tests that comment trivia is preserved in code actions

// Leading comment on a simple rename
// test -> it
test("with leading comment", () => {});

// Trailing comment should be preserved
test("with trailing comment", () => {}); // trailing comment

/* Block comment before call */
test.skip("block comment before skip", () => {});

// Leading comment on member expression
test.only("with leading comment on only", () => {}); // trailing

// Comment preserved for xtest -> xit
xtest("xtest with comment", () => {}); // should become xit

// Comment preserved for fit -> it.only
fit("fit with comment", () => {}); // should become it.only

// Comment between identifier and call parens (same line — trailing trivia of identifier)
test /* comment between ident and paren */ ("inline block comment", () => {});
test/* comment on member */.skip("inline comment on skip", () => {});
fit /* comment on fit */ ("inline comment on fit", () => {});

// Inside describe: comments preserved too
describe("suite", () => {
  // Leading comment inside describe
  test("inner test with comment", () => {});
  test.skip("inner skip with comment", () => {}); // trailing in describe
  // Block comment before only inside describe
  test.only("inner only with comment", () => {});
  fit("inner fit with comment", () => {}); // trailing on fit inside describe
});
