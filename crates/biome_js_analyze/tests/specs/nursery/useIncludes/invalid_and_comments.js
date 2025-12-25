// should generate diagnostics

// indexOf with comments
"foo".indexOf("o") /* comment 0*/ !== -1;
/* comment1 */ "foo".indexOf("o") !== -1;
"foo"/* comment2 */.indexOf("o") !== -1;
("foo".indexOf("o")) /* comment 3*/ !== -1;

// Regex with comments
/* comment */ /a/.test("abc");
/a/.test("abc") /* comment */;
/a//* comment */.test("abc");
