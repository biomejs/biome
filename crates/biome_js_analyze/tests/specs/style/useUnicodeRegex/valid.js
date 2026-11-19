/* should not generate diagnostics */

// Regex literals with u flag
/foo/u;
/bar/iu;
/baz/giu;
/qux/gimu;
/test/gimuy;

// Regex literals with v flag
/foo/v;
/bar/iv;
/baz/giv;
/qux/gimv;
/test/gimvy;

// Unicode patterns with u flag (the main use case!)
/😀/u;
/\u{1F600}/u;
/[\u{1F600}-\u{1F64F}]/u;
/\p{Emoji}/u;

// Unicode patterns with v flag
/😀/v;
/\p{Emoji}/v;
/[\p{Emoji}--\p{ASCII}]/v;

// RegExp constructor with u flag
RegExp("foo", "u");
RegExp("foo", "gu");
RegExp("foo", "giu");
RegExp("foo", "gimu");
RegExp("foo", "gimuy");
new RegExp("foo", "u");
new RegExp("foo", "iu");
new RegExp("foo", "giu");
new RegExp("foo", "gimuy");

// RegExp constructor with v flag
RegExp("foo", "v");
RegExp("foo", "gv");
RegExp("foo", "giv");
RegExp("foo", "gimv");
RegExp("foo", "gimvy");
new RegExp("foo", "v");
new RegExp("foo", "iv");
new RegExp("foo", "giv");
new RegExp("foo", "gimvy");

// Unicode in RegExp constructor
new RegExp("\u{1F600}", "u");
new RegExp("\p{Emoji}", "u");
new RegExp("😀", "u");

// Dynamic flags - should be ignored (cannot statically analyze)
new RegExp("foo", flags);
RegExp("foo", flags);
new RegExp("foo", getFlags());
RegExp("foo", getFlags());

// Shadowed RegExp - should be ignored
function test(RegExp) { return new RegExp("foo"); }

// globalThis.RegExp with u/v flag
globalThis.RegExp("foo", "u");
globalThis.RegExp("foo", "v");
new globalThis.RegExp("foo", "u");
new globalThis.RegExp("foo", "giv");

// window.RegExp with u/v flag
window.RegExp("foo", "u");
window.RegExp("foo", "v");
new window.RegExp("foo", "u");
new window.RegExp("foo", "giv");
