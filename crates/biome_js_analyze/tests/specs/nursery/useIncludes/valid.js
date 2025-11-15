/* should not generate diagnostics */

"foo".includes("o");
!["a", "b", "c"].includes("a");
"abc".includes("a");

// These are valid because they are not the simple checks we are looking for
"foo".indexOf("o") > 0;
"foo".indexOf("o") === 1;

// The rule should not trigger if the receiver is not known to have `includes`
let obj = { indexOf: () => 0 };
obj.indexOf("a") !== -1;

/a/i.test("abc");
