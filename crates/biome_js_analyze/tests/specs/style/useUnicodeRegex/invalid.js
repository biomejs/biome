// Regex literals without u/v flag
/foo/;
/bar/i;
/baz/gi;
/qux/gim;
/test/gimy;

// Unicode patterns WITHOUT u/v flag - this is the problem!
/😀/;
/café/i;

// RegExp constructor without flags
RegExp("foo");
new RegExp("foo");

// RegExp constructor with trailing comma (no flags)
new RegExp("foo",);

// RegExp constructor with non-unicode flags
RegExp("foo", "");
RegExp("foo", "g");
RegExp("foo", "gi");
RegExp("foo", "gim");
RegExp("foo", "gimy");
new RegExp("foo", "");
new RegExp("foo", "i");
new RegExp("foo", "gi");
new RegExp("foo", "gimy");

// Single quotes - should preserve quote style in fix
new RegExp("foo", 'gi');
RegExp("foo", 'gim');

// Unicode in RegExp constructor WITHOUT u/v flag
new RegExp("😀");
new RegExp("😀", "g");

// Parenthesized pattern
new RegExp(("foo"));
new RegExp(("foo"), "gi");

// globalThis.RegExp
globalThis.RegExp("foo");
globalThis.RegExp("foo", "gi");
new globalThis.RegExp("foo");
new globalThis.RegExp("foo", "gi");

// window.RegExp (browser)
window.RegExp("foo");
window.RegExp("foo", "gi");
new window.RegExp("foo");
new window.RegExp("foo", "gi");

// Trivia preservation in constructor flags
new RegExp("foo", /* leading */ "gi" /* trailing */);

// Patterns that stay valid with the u flag - should be fixed
/a{2}/;
/a{1,}b{1,3}?/g;
/[\d-]/;
/[-\w]/;
/[a-z\-]/;
/(a)\1/;
/(?<n>a)\k<n>/;
/\cA\x41\u0041\0/;
/[\w\s\b]/;
/(?=a)b/;
/(?<!a)b/;
/\/\.\*/;
/[^\]]/;
new RegExp("\\d+", "g");

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
