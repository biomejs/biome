// generally invalid
/(b)(\2a)/;
/\k<foo>(?<foo>a)/;
RegExp("(a|bc)|\\1");
new RegExp("(?!(?<foo>\\n))\\1");
/(?<!(a)\1)b/;

// nested
new RegExp("(\\1)");
/^(a\1)$/;
/^((a)\1)$/;
new RegExp("^(a\\1b)$");
RegExp("^((\\1))$");
/((\2))/;
/a(?<foo>(.)b\1)/;
/a(?<foo>\k<foo>)b/;
/^(\1)*$/;
/^(?:a)(?:((?:\1)))*$/;
/(?!(\1))/;
/a|(b\1c)/;
/(a|(\1))/;
/(a|(\2))/;
/(?:a|(\1))/;
/(a)?(b)*(\3)/;
/(?<=(a\1))b/;

// forward
/\1(a)/;
/\1.(a)/;
/(?:\1)(?:(a))/;
/(?:\1)(?:((a)))/;
/(?:\2)(?:((a)))/;
/(?:\1)(?:((?:a)))/;
/(\2)(a)/;
RegExp("(a)\\2(b)");
/(?:a)(b)\2(c)/;
/\k<foo>(?<foo>a)/;
/(?:a(b)\2)(c)/;
new RegExp("(a)(b)\\3(c)");
/\1(?<=(a))./;
/\1(?<!(a))./;
/(?<=\1)(?<=(a))/;
/(?<!\1)(?<!(a))/;
/(?=\1(a))./;
/(?!\1(a))./;

// backward in the same lookbehind
/(?<=(a)\1)b/;
/(?<!.(a).\1.)b/;
/(.)(?<!(b|c)\2)d/;
/(?<=(?:(a)\1))b/;
/(?<=(?:(a))\1)b/;
/(?<=(a)(?:\1))b/;
/(?<!(?:(a))(?:\1))b/;
/(?<!(?:(a))(?:\1)|.)b/;
/.(?!(?<!(a)\1))./;
/.(?=(?<!(a)\1))./;
/.(?!(?<=(a)\1))./;
/.(?=(?<=(a)\1))./;

// into another alternative
/(a)|\1b/;
/^(?:(a)|\1b)$/;
/^(?:(a)|b(?:c|\1))$/;
/^(?:a|b(?:(c)|\1))$/;
/^(?:(a(?!b))|\1b)+$/;
/^(?:(?:(a)(?!b))|\1b)+$/;
/^(?:(a(?=a))|\1b)+$/;
/^(?:(a)(?=a)|\1b)+$/;
/.(?:a|(b)).|(?:(\1)|c)./;
/.(?!(a)|\1)./;
/.(?<=\1|(a))./;

// into a negative lookaround
/a(?!(b)).\1/;
/(?<!(a))b\1/;
/(?<!(a))(?:\1)/;
/.(?<!a|(b)).\1/;
/.(?!(a)).(?!\1)./;
/.(?<!(a)).(?<!\1)./;
/.(?=(?!(a))\1)./;
/.(?<!\1(?!(a)))/;

// valid and invalid
/\1(a)(b)\2/;
/\1(a)\1/;

// multiple invalid
/\1(a)\2(b)/;
/\1.(?<=(a)\1)/;
/(?!\1(a)).\1/;
/(a)\2(b)/;
RegExp("(\\1)");

// when flags cannot be evaluated, it is assumed that the regex doesn't have 'u' flag set, so this will be a correct regex with a useless backreference
RegExp("\\1(a){", flags);

// able to evaluate some statically known expressions
const r = RegExp;
const p = "\\1";
const s = "(a)";
new r(p + s);
