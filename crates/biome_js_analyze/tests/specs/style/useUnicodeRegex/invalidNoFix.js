// RegExp constructor without a flags argument - no fix, it would need a new argument
RegExp("foo");
new RegExp("foo");
new RegExp("foo",);
new RegExp("😀");
new RegExp(("foo"));
globalThis.RegExp("foo");
new globalThis.RegExp("foo");
window.RegExp("foo");
new window.RegExp("foo");

// Patterns that are invalid with the u flag - should not be fixed
/{/;
/}/;
/]/;
/a{/;
/a{,5}/;
/\a/;
/\-/;
/\k/;
/[\d-z]/;
/(?=a)*/;
/\c1/;
/[\c1]/;
/\1/;
/\00/;
/(a)[\1]/;
/(a)(b)[\2]/;
/[\B]/;
/[\uD83D\uDE00-\uDE01]/;
/[😀-\uFFFF]/;
new RegExp("\\-", "g");
new RegExp("{", "g");

// Patterns whose meaning changes with the u flag - should not be fixed
/\p{L}/;
/\u{41}/;

// Dynamic pattern - should not be fixed
new RegExp(pattern, "g");
