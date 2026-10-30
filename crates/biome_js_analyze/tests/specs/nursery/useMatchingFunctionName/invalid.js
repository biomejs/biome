/* should generate diagnostics */
let foo1 = function bar() {};
var foo2 = (function bar() {});
foo3 = function bar() {};
foo4 &&= function bar() {};
obj.foo5 ||= function bar() {};
obj["foo6"] ??= function bar() {};
obj.foo7 = function bar() {};
obj.bar.foo8 = function bar() {};
obj["foo9"] = function bar() {};
(obj?.aaa).foo10 = function bar() {};
foo11 = (0, function bar() {});
let obj1 = { foo12: function bar() {} };
let obj2 = { "foo13": function bar() {} };
let obj3 = { ["foo14"]: function bar() {} };
let obj4 = { "ᢅ": function bar() {} };
Object.defineProperty(foo, "bar", { value: function baz() {} });
class C1 {
	foo15 = function bar() {};
	"foo16" = function bar() {};
	["foo17"] = function bar() {};
	static foo18 = function bar() {};
}
(class {
	foo19 = function bar() {};
});
let foo20 = async function* bar() {};
(foo21) = function bar() {};
((obj.foo22)) = function bar() {};
obj.static = function bar() {};
let obj5 = { public: function bar() {} };
