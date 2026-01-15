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
