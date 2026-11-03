/* should not generate diagnostics */
a.charAt(4) === 'a';
a.charAt(4) === '\n';
a.charAt(4) === '	';
a.charAt(4) === '﻿';
a.charAt(4) !== '﻿';
a.charAt(4) === '';
a.charAt(4) === ``;
a.charAt(4) === `a`;
a.charAt(4) === '\x41';
a.charAt(4) === 'A';
a.charAt(4) === '\u{41}';
a.charAt(4) === '\u{FFFF}';
a.charAt(4) === '\\';
a.charAt(4) === 'a\
';
a.charAt(4) === `${b}c`;
a.charAt(4) === b;
a.charAt(4) < 'ab';
a.charAt(4) + 'ab';
"abc".charAt() === "ab";
"abc".charAt(0, 1) === "ab";
"abc".charAt(...args) === "ab";
a?.charAt(0) === "ab";
a.charAt?.(0) === "ab";
a[charAt](0) === "ab";
[1, 2].at(0) === "ab";
a.at(0) === "ab";
"abc".slice(0) === "ab";
chatAt(4) === 'a2';
new chatAt(4) === 'a';
"abc"[b] === "ab";
"abc"["length"] === "ab";
"abc"["01"] === "ab";
"abc"[-1] === "ab";
tag`abc`[0] === "ab";
`abc`.at(0) === "ab";
`abc`[0] === "ab";
"abc"["9007199254740992"] === "ab";
{
	const value = [1, 2];
	value[0] === "ab";
}
{
	const value = 100;
	value[0] === "ab";
}
{
	const value = `abc`;
	value[0] === "ab";
}
{
	let value = "abc";
	value[0] === "ab";
}
{
	const value = getValue();
	value.at(0) === "ab";
}
{
	const value = "abc";
	value?.[0] === "ab";
}
