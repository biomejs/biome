/* should not generate diagnostics */
var o1 = { get a() {} };
var o2 = { get a() {}, set a(value) {} };
var o3 = { set a(value) {}, get a() {} };
var o4 = { set a(value) {}, b: 1, get a() {} };
var o5 = { get 'a'() {}, set a(value) {} };
var o6 = { get [`a`]() {}, set a(value) {} };
var o7 = { get [1]() {}, set 1(value) {} };
var o8 = { get [a]() {}, set [a](value) {} };
var o9 = { get [a.b]() {}, set [a. b](value) {} };
var o10 = { a: 1, set: 2 };
var o11 = { set() {} };
class C1 { get a() {} set a(value) {} }
class C2 { static get a() {} static set a(value) {} }
class C3 { get #a() {} set #a(value) {} }
class C4 { get [a]() {} set [a](value) {} }
class C5 { get a() {} }
Object.defineProperty(obj, "a", { get() {}, set(value) {} });
Object.defineProperty(obj, "a", { get: function () {}, set: function (value) {} });
Object.defineProperty(obj, "a", { get, set });
Object.defineProperty(obj, "a", { value: 1 });
Object.defineProperty(obj, "a", { ...base, set(value) {} });
Object.defineProperty(obj, "a", { ["set"]: function (value) {} });
Object.defineProperties(obj, { a: { get() {}, set(value) {} } });
Object.create(null, { a: { get() {}, set(value) {} } });
// Not a property descriptor
foo({ set(value) {} });
Object.defineProperty({ set(value) {} });
Object.create({ set(value) {} });
Object.foo(obj, "a", { set(value) {} });
Foo.defineProperty(obj, "a", { set(value) {} });
function shadowed(Object) {
	Object.defineProperty(obj, "a", { set(value) {} });
}
