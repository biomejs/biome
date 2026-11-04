/* should not generate diagnostics */
"".matchAll(/a/g);
"".matchAll(/a/giu);
"".matchAll(new RegExp("a", "g"));
"".matchAll(RegExp("a", "g"));
"".matchAll(globalThis.RegExp("a", "g"));
"".matchAll(new RegExp(/a/g));

// Arguments that aren't regular expressions, or whose flags are unknown.
"".matchAll("a");
"".matchAll();
"".matchAll(...args);
"".matchAll(pattern);
"".matchAll(new RegExp("a", flags));
"".matchAll(new RegExp("a", `${flags}`));
"".matchAll(new RegExp(source));
"".matchAll(new RegExp(...args));
"".matchAll(new RegExp(...parts, "i"));
"".matchAll(new Foo("a"));

// Not a `matchAll` method call.
matchAll(/a/);
new matchAll(/a/);
"".match(/a/);
"".replaceAll(/a/g, "b");
"".matchAll;

// `RegExp` is a local variable.
function test1(RegExp) {
	"".matchAll(new RegExp("a"));
}

// Variables that aren't `const`, or that aren't regular expressions.
let pattern1 = /a/;
"".matchAll(pattern1);

const pattern2 = "a";
"".matchAll(pattern2);

const pattern3 = /a/g;
"".matchAll(pattern3);

const pattern4 = new RegExp("a", "g");
"".matchAll(pattern4);

const pattern5 = new RegExp("a", condition ? "g" : "gi");
"".matchAll(pattern5);

const { pattern6 } = { pattern6: /a/ };
"".matchAll(pattern6);

const a = b;
const b = a;
"".matchAll(a);

const pattern7 = pattern7;
"".matchAll(pattern7);

// Variables from destructuring patterns hold part of the value.
const re = /a/;
const { source } = re;
"".matchAll(source);

const { source: source2 } = /a/;
"".matchAll(source2);
