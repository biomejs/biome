/* should generate diagnostics */
console.log("abc ", "def");
console.log("abc", " def");
console.log(" abc ", "def");
console.debug("abc ", "def");
console.info("abc ", "def");
console.warn("abc ", "def");
console.error("abc ", "def");
console.log("abc", " def ", "ghi");
console.log("abc ", "def ", "ghi");
console.log('abc ', "def");
console.log(`abc `, "def");
console.log(`abc ${1 + 2} `, "def");
console.log("abc", ` ${foo} def`);
console.log("abc", ` ${foo}`);
console.log("abc", `${foo} `, "def");
console.log(
	'abc',
	'def ',
	'ghi'
);
console.error(
	theme.error('✗'),
	'Verifying "packaging" fixture\\n ',
	theme.error(errorMessage)
);
// The space follows an escaped backslash
console.log("a\\ ", "def");
console.log(("abc "), "def");
console.log("é ", "def");
(console).log("abc ", "def");
(console.log)("abc ", "def");
