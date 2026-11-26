/* should generate diagnostics */
var o1 = { set a(value) {} };
var o2 = { set a(value) {}, get b() {} };
var o3 = { set 'a'(value) {} };
var o4 = { set [a](value) {}, get [b]() {} };
var o5 = { set [a.b](value) {}, get [a.c]() {} };
var o6 = { set a(value) {}, b: 1, a: 2 };
var o7 = { get a() {}, inner: { set a(value) {} } };
var o8 = { ...base, set a(value) {} };
class C1 { set a(value) {} }
class C2 { static set a(value) {} }
class C3 { static set a(value) {} get a() {} }
class C4 { set a(value) {} static get a() {} }
class C5 { set #a(value) {} get a() {} }
class C6 { set [a](value) {} get [b]() {} }
var C7 = class { set a(value) {} };
class C8 { @dec set a(value) {} }
Object.defineProperty(obj, "a", { set: function (value) {} });
Object.defineProperty(obj, "a", { set(value) {} });
Object.defineProperty(obj, "a", { set });
Object.defineProperty(obj, "a", { "set": function (value) {} });
Object.defineProperty(obj, "a", ({ set(value) {} }));
Object?.defineProperty(obj, "a", { set(value) {} });
globalThis.Object.defineProperty(obj, "a", { set(value) {} });
Reflect.defineProperty(obj, "a", { set(value) {} });
Object.defineProperties(obj, { a: { set(value) {} } });
Object.create(null, { a: { set(value) {} } });
