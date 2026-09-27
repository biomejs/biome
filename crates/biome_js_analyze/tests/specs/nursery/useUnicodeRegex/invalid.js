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

// Patterns invalid in Unicode mode: the diagnostic is reported but the
// `u`-flag fix must not be offered, otherwise it produces a SyntaxError.
/{/;
/]/;
/}/;
/a{/;
/a}/;
/a{b}/;
/a{,2}/;
/a{2,/;
/\a/;
/\-/;
/\k/;
/\q/;
/[\d-z]/;
/[a-\d]/;
/\x1/;
/\u004/;
/\u{}/;
/\1/;
/\01/;
/\08/;
/[]]/;
new RegExp("{");
RegExp("\\a", "g");
new RegExp("[\\d-z]", "gi");

// Unicode property escapes: valid with the `u` flag, but adding it changes
// the meaning (from the literal text to a Unicode property), so the fix is
// withheld.
/\p{L}/;
/\P{L}/;

// Lazy quantifiers are valid in Unicode mode, so the fix is offered.
/a+?/;
/(?:ab){2,3}?/;
