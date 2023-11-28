/^abc[]/;
/foo[]bar/;
if (foo.match(/^abc[]/)) {}
if (/^abc[]/.test(foo)) {}
/[]]/;
/\[[]/;
/\[\[\]a-z[]/;
/[]]/d;
/[(]\u{0}*[]/u;
/[]/v;
/[[]]/v;
/[[a][]]/v;
/[a[[b[]c]]d]/v;
/[a--[]]/v;
/[[]--b]/v;
/[a&&[]]/v;
/[[]&&b]/v;
// negated empty class
/[^]/;
/\[][^]/;