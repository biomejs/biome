/\ /;
/[\-ab]/;
/[ab\?]/;
/[ab\.]/;
/[a\|b]/;
/\-/;
/[\-]/;
/[\-]/;
/[\(paren]/;
/[\[]/;
/[\/]/; // A character class containing '/'
/[\B]/;
/[a][\-b]/;
/\-[]/;
/[a\^]/;
/[^\^]/;
/[^\^]/u;
/[\$]/v;
/[\*\*]/v;
/[\+\+]/v;
/[\?\?]/v;
/[^\^\^]/v;
/[_\^\^]/v;
/[\p{ASCII}--\.]/v;
/[\p{ASCII}&&\.]/v;
/[\.--[.&]]/v;
/[\.&&[.&]]/v;
/[\.--\.--\.]/v;
/[\.&&\.&&\.]/v;
/[[\.&]--[\.&]]/v;
/[[\.&]&&[\.&]]/v;

// Unlike ESLint, we report `\k` when it is not in a unicode-aware regex
/(?<a>)\k<a>/;

// A test with unicode characters that take more than one byte
/😀\😀/

// https://github.com/biomejs/biome/issues/6201
// Three or more backslashes before a trailing dash: an escape pair leaves a
// `\-` that's still an useless escape, so the rule should report it.
/[\\\-]/;
/[\\\\-]/;