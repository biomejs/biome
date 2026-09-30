// Regex literals without u/v flag
/foo/;
/bar/i;
/baz/gi;
/qux/gim;
/test/gimy;

// Unicode patterns WITHOUT u/v flag - this is the problem!
/😀/;
/café/i;

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
new RegExp("😀", "g");

// Parenthesized pattern
new RegExp(("foo"), "gi");

// globalThis.RegExp
globalThis.RegExp("foo", "gi");
new globalThis.RegExp("foo", "gi");

// window.RegExp (browser)
window.RegExp("foo", "gi");
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
/\k<n>(?<n>a)/;
/\cA\x41\u0041\0/;
/[\w\s\b]/;
/(?=a)b/;
/(?<!a)b/;
/\/\.\*/;
/[^\]]/;
new RegExp("\\d+", "g");
// An escaped backslash followed by a digit is not a legacy octal escape
new RegExp("\\1(a)", "g");
