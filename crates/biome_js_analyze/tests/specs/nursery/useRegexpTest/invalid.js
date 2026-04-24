if (str.match(/test/)) {}
if (/test/.exec(str)) {}
if (str.match(/test/i)) {}
while (str.match(/test/)) {}
for (; str.match(/test/);) {}
do {} while (str.match(/test/));
if (obj.prop.match(/test/)) {}
if (/test/.exec(obj.prop)) {}
if (str.match(/a/)) {} if (/b/.exec(str)) {}
if (str.match(new RegExp('test'))) {}
if (new RegExp('test').exec(str)) {}
if (str.match(new window.RegExp('test'))) {}
if (str.match(new globalThis.RegExp('test'))) {}
const x = str.match(/test/) ? a : b;
if (!str.match(/test/)) {}
if (str.match(/test/) && other) {}
if (other && str.match(/test/)) {}
if (str.match(/test/) || fallback) {}
if (fallback || str.match(/test/)) {}
if (a && !str.match(/test/)) {}
if (!!str.match(/test/)) {}

// trivia is preserved for code actions
if (
	str.match(
			// comment
			/test/)
) {}
