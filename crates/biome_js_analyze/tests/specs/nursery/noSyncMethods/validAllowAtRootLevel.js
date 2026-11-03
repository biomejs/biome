/* should not generate diagnostics */
fs.readFileSync(somePath).toString();
if (true) { fs.fooSync(); }
if (true) { fooSync(); }
var foo = fooSync.call(null);
class A {
	static { fs.fooSync(); }
	static staticProperty = fs.fooSync();
}
class B {
	[keySync()]() {}
	[otherKeySync()] = 1;
	@decorator(readFileSync("x")) method() {}
}
var c = { [keySync()]() {} };
