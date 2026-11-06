/* should generate diagnostics */
var foo = fs.fooSync();
var foo = fooSync();
var foo = fs.fooSync.apply();
var foo = fooSync.apply();
var foo = fooSync.call(null, "path");
var foo = fs?.fooSync();
var foo = fs["fooSync"]();
var foo = (fs.fooSync)();
fs.readFileSync(somePath).toString();
if (true) { fs.fooSync(); }
function someFunction() { fs.fooSync(); }
var a = function someFunction() { fs.fooSync(); };
var b = () => fooSync();
class A {
	method() { fs.fooSync(); }
	static { fs.fooSync(); }
	property = fs.fooSync();
	static staticProperty = fs.fooSync();
}
