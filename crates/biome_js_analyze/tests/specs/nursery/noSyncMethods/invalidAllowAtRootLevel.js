/* should generate diagnostics */
function someFunction() { fs.fooSync(); }
var a = function someFunction() { fs.fooSync(); };
var b = () => fs.readFileSync(qux);
var c = { method() { fooSync(); }, get value() { return fooSync(); } };
class A {
	constructor() { fooSync(); }
	method() { fs.fooSync(); }
	set value(v) { fooSync(); }
	property = fs.fooSync();
}
function withDefault(value = fooSync()) {}
var d = (value = fooSync()) => value;
class E { set value(v = fooSync()) {} }
var f = { set value(v = fooSync()) {} };
