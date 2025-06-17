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