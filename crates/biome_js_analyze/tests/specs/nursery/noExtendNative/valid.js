/* should not generate diagnostics */

// Subclassing a built-in is the right approach.
class CustomArray extends Array {}

// Extending your own object is fine.
const obj = {};
obj.extra = "a";
MyClass.prototype.method = function () {};

// A non-prototype static property of a built-in is out of scope.
Array.from2 = function () {};

// defineProperty on a non-prototype target.
Object.defineProperty(obj, "x", { value: 1 });
