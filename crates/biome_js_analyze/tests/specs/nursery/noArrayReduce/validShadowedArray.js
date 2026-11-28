/* should not generate diagnostics */
const Array = { prototype: { reduce: { call() {} } } };
Array.prototype.reduce.call(foo, fn);
