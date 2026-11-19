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

// Parenthesized prototype expression.
(Array.prototype).times = 1;
(Object["prototype"]).x = 2;

// Parenthesized object with prototype access.
(Array).prototype.times = 1;

// Computed method name on defineProperty.
Object["defineProperty"](String.prototype, "x", { value: 1 });

// Parenthesized callee object.
(Object).defineProperty(Number.prototype, "x", { value: 1 });

// WeakRef and FinalizationRegistry.
WeakRef.prototype.foo = 1;
FinalizationRegistry.prototype.foo = 1;
