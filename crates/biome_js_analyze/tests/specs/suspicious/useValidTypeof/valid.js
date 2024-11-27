typeof foo === "string";
typeof foo === "object";
typeof foo === "function";
typeof foo === "undefined";
typeof foo === "boolean";
typeof foo === "number";
typeof foo === "bigint";
"string" === typeof foo;
"object" === typeof foo;
"function" === typeof foo;
"undefined" === typeof foo;
"boolean" === typeof foo;
"number" === typeof foo;

typeof foo == "string";
typeof(foo) === "string";
typeof(foo) !== "string";
typeof(foo) == "string";
typeof(foo) != "string";

typeof foo === typeof bar;
typeof foo === baz;
typeof foo !== someType;
typeof bar != someType;
someType === typeof bar;
someType == typeof bar;
var oddUse = typeof foo + "thing";
function f(undefined) { typeof x === undefined }

typeof foo === f();
typeof foo == (f() ?? g());
typeof foo === obj.prop;
typeof foo === obj["prop"];
class C {
	f() {
		typeof foo === this.prop;
		typeof foo === super.f();
	}
}
