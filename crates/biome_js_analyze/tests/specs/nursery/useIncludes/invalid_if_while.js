// should generate diagnostics

// in if/while
if ("foo".indexOf("o") !== -1) {}
while (["a", "b", "c"].indexOf("a") === -1) {}
if (("foo".indexOf("o")) !== -1) {}

// With parentheses
("hello".indexOf("e")) !== -1;

// Basic regex patterns
/a/.test("abc");

// Unicode characters in regex
/á/.test("café");
/😀/.test("hello 😀");
/中/.test("中文");

// Whitespace in regex
/ /.test("hello world");
