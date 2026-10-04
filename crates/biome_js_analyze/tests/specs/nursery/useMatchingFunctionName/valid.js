/* should not generate diagnostics */
var foo;
var foo1 = function foo1() {};
var foo2 = function () {};
var foo3 = () => {};
foo4 = function foo4() {};
foo5 &&= function foo5() {};
obj.foo6 ||= function foo6() {};
obj["foo7"] ??= function foo7() {};
obj.foo8 = function foo8() {};
obj.foo9 = function () {};
obj.bar.foo10 = function foo10() {};
obj["foo11"] = function foo11() {};
obj["foo//bar"] = function foo() {};
obj[foo] = function bar() {};
obj["x" + 2] = function bar() {};
obj[1] = function bar() {};
obj.class = function klass() {};
obj["default"] = function fallback() {};
var obj15 = { delete: function remove() {} };
foo12 += function bar() {};
var obj1 = { foo13: function foo13() {} };
var obj2 = { "foo14": function foo14() {} };
var obj3 = { "foo//bar": function foo() {} };
var obj4 = { foo15: function () {} };
var obj5 = { [foo]: function bar() {} };
var obj6 = { ["x" + 2]: function bar() {} };
var obj7 = { ["foo16"]: function foo16() {} };
var obj8 = { ["❤"]: function foo() {} };
var obj9 = { [null]: function foo() {} };
var obj10 = { [1]: function foo() {} };
var obj11 = { 1: function foo() {} };
var obj12 = { foo17: (function bar() {})() };
var obj13 = { foo18() {} };
var obj14 = { "foo19": function foo19() {} };
var [bar] = [function bar() {}];
var { baz } = function foo() {};
[] = function foo() {};
({ a } = function foo() {});
function a(foo = function bar() {}) {}
foo20(function bar() {});
module.exports = function foo() {};
module["exports"] = function foo() {};
class C1 {
	x = function () {};
	"x" = function () {};
	#x = function () {};
	[x] = function () {};
	["x"] = function () {};
	x1 = function x1() {};
	"x2" = function x2() {};
	#x3 = function y() {};
	[x4] = function y() {};
	["x5"] = function x5() {};
	"xy " = function foo() {};
	1 = function x0() {};
	[f()] = function g() {};
	static x6 = function x6() {};
	x7 = (function y() {})();
}
class C2 {
	#x;
	foo() {
		this.#x = function y() {};
		a.b.#x = function y() {};
	}
}
