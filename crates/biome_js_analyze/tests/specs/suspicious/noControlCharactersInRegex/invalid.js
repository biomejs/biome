RegExp("\\x1f");
RegExp("\\u{1111}*\\x1F", "u");
new RegExp("\\x1f\\x1e");
new RegExp("\\x1fFOO\\x00");
new RegExp("FOO\\x1fFOO\\x1f");
new RegExp("\\x1f");
new RegExp("\\u001F", flags);
new RegExp("\\u{1111}*\\x1F", "u");
new RegExp("\\u{1F}", "u");
new RegExp("\u{1F}");
new RegExp("\\u{1F}", "gui");
new RegExp("\\x0C");
new RegExp("\x0C");
new RegExp("	");; // tab
new RegExp("\	");; // escaped tab
new RegExp("\\	");; // escaped tab
/\x00/;
/\x0C/;
/\x1F/;
/\u000C/;
/\u{C}/u;
/\\\x1f\\x1e/;
/\\\x1fFOO\\x00/;
/FOO\\\x1fFOO\\x1f/;
/(?<a>\\x1f)/;
/(?<\u{1d49c}>.)\x1f/;
/\u{1111}*\x1F/u;
/\u{1111}*\x1F/v;
/\u{1F}/u;
/\u{1F}/gui;
/\u000C\n/u;
/	/u; // tab
/\	/u; // escaped tab
// Edge cases
/\x1\x1f/g;
/\u001\u000C/g;