/^(?:(a)|(b)\2)$/; // reference to (b)

/(a)\1/; // reference to (a)

RegExp("(a)\\1(b)"); // reference to (a)

/(a)(b)\2(c)/; // reference to (b)

/(?<foo>a)\k<foo>/; // reference to (?<foo>a)

/(?<=\1(a))b/; // reference to (a), correctly before the group as they're in the same lookbehind

/(?<=(a))b\1/; // reference to (a), correctly after the group as the backreference isn't in the lookbehind

new RegExp("(.)\\1"); // reference to (.)

/^(?:(a)\1)$/; // reference to (a)

/^((a)\2)$/; // reference to (a)

/a(?<foo>(.)b\2)/; // reference to (.)

/a(?!(b|c)\1)./; // reference to (b|c), correct as it's from within the same negative lookahead

/(?<!\1(a))b/; // reference to (a), correct as it's from within the same negative lookbehind

// comments describe behavior in a browser
/^[\1](a)$/.test("\x01a"); // true. In a character class, \1 is treated as an octal escape sequence.
/^\1$/.test("\x01"); // true. Since the group 1 doesn't exist, \1 is treated as an octal escape sequence.
/^(a)\1\2$/.test("aa\x02"); // true. In this case, \1 is a backreference, \2 is an octal escape sequence.
