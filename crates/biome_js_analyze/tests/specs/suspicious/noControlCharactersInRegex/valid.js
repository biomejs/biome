RegExp("x1f");
RegExp("[");
new RegExp("x1f");
new RegExp("[");
new RegExp("\\u{20}", "u");
new RegExp("\\u{1F}");
new RegExp("\\u{1F}", "g");
new RegExp("\\u{1F}", uflags);
new RegExp("\t");
new RegExp("\n");
/\t/;
/\n/;
/x1f/;
/\\x1f/;
/\u{20}/u;
/\u{1F}/;
/\u{1F}/g;
/\t/;
/\n/;
/\x/;
/\u/;
new (function foo() {})("\\x1f");
/[\u200E\u2066-\u2069]/gu;

// https://github.com/biomejs/biome/issues/4565
/\u/u