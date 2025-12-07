// indexOf with comments
"foo".indexOf("o") /* comment */ !== -1;
/* comment */ "foo".indexOf("o") !== -1;
"foo"/* comment */.indexOf("o") !== -1;
("foo".indexOf("o")) /* comment */ !== -1;

// Regex with comments
/* comment */ /a/.test("abc");
/a/.test("abc") /* comment */;
/a//* comment */.test("abc");
