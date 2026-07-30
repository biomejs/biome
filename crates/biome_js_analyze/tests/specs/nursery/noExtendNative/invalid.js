/* should generate diagnostics */

// Direct prototype assignment.
Object.prototype.extra = "a";
Array.prototype.times = function () {};
String.prototype["x"] = 1;

// Computed prototype access via bracket on prototype property.
Number["prototype"]["y"] = 2;

// Object.defineProperty / defineProperties on a native prototype.
Object.defineProperty(Array.prototype, "times", { value: 999 });
Object.defineProperties(String.prototype, { foo: { value: 1 } });
