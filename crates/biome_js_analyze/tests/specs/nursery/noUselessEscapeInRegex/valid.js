/[\d]/;
/[a\-b]/;
/foo\?/;
/example\.com/;
/foo\|bar/;
/\^bar/;
/[\^bar]/;
/\(bar\)/;
/[[\]]/; // A character class containing '[' and ']'
/[[]\./; // A character class containing '[', followed by a '.' character
/[\]\]]/; // A (redundant) character class containing ']'
/\[abc]/; // Matches the literal string '[abc]'
/\[foo\.bar]/; // Matches the literal string '[foo.bar]'
/vi/m;
/\B/;

// https://github.com/eslint/eslint/issues/7472
/\0/; // null character
/\1/; // \x01 character (octal literal)
/(a)\1/; // backreference
/(a)\12/; // backreference
/(a)\9/; // backreference
/[\0]/; // null character in character class

// https://github.com/eslint/eslint/issues/7789
/]/;
/\]/;
/foo\]/;
/[[]\]/; // A character class containing '[', followed by a ']' character
/\[foo\.bar\]/;

// ES2018
/\]/u;
// /(?<a>)\k<a>/; // Unlike ESLint, we report `\k` when it is not in a unicode-aware regex
/(\?<a>)/;
/\p{ASCII}/u;
/\P{ASCII}/u;
/[\p{ASCII}]/u;
/[\P{ASCII}]/u;

// Carets
/[^^]/;
/[^^]/u;

// ES2024
/[\q{abc}]/v;
/[\(]/v;
/[\)]/v;
/[\{]/v;
/[\]]/v;
/[\}]/v;
/[\/]/v;
/[\-]/v;
/[\|]/v;
/[\$$]/v;
/[\&&]/v;
/[\!!]/v;
/[\##]/v;
/[\%%]/v;
/[\**]/v;
/[\++]/v;
/[\,,]/v;
/[\..]/v;
/[\::]/v;
/[\;;]/v;
/[\<<]/v;
/[\==]/v;
/[\>>]/v;
/[\??]/v;
/[\@@]/v;
/[\``]/v;
/[\~~]/v;
/[^\^^]/v;
/[_\^^]/v;
/[$\$]/v;
/[&\&]/v;
/[!\!]/v;
/[#\#]/v;
/[%\%]/v;
/[*\*]/v;
/[+\+]/v;
/[,\,]/v;
/[.\.]/v;
/[:\:]/v;
/[;\;]/v;
/[<\<]/v;
/[=\=]/v;
/[>\>]/v;
/[?\?]/v;
/[@\@]/v;
/[`\`]/v;
/[~\~]/v;
/[^^\^]/v;
/[_^\^]/v;
/[\&&&\&]/v;
/[[\-]\-]/v;
/[\^]/v;

/[z-]/;

// Edge case
/[]/;
