/* should generate diagnostics */
a.charAt(4) === 'aa';
a.charAt(4) === '/n';
a.charAt(3) === '/t';
a.charAt(4) === 'ac';
a.charAt(822) !== 'foo';
a.charAt(4) == 'ab';
a.charAt(4) != 'ab';
'ab' === a.charAt(4);
a.charAt(4) === '\n\n';
a.charAt(4) === '\x41b';
a.b.charAt(4) === 'ab';
"abc".at(0) === "ab";
"abc".at(-1) !== "ab";
"abc"[0] === "ab";
"abc"["0"] === "ab";
"abc"[10] === "ab";
"abc"["9007199254740991"] === "ab";
"abc".charAt(0) === `ab`;
("abc".charAt(0)) === "ab";
"abc".charAt(0) === ("ab");
"abc".charAt(0) === "😀";
"abc".charAt(0) === "\u{1F600}";
"abc".charAt(0) === "\uD83D\uDE00";
{
	const value = "abc";
	value[0] === "ab";
}
{
	const value = "abc";
	value.at(0) === "ab";
}
