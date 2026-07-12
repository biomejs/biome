/* should generate diagnostics */

// Direct prototype assignment.
Object.prototype.extra = "a";

Array.prototype.times = function () {};

Error.prototype.log = () => {};

// Computed prototype assignment.
Number.prototype["double"] = function () {};

// Object.defineProperty / defineProperties on a native prototype.
Object.defineProperty(Array.prototype, "times", { value: 999 });

Object.defineProperties(String.prototype, { foo: { value: 1 } });
