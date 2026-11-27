/* should not generate diagnostics */
if (x === null) {}
if (typeof arr === "string") {}
if (item === 5) {}
if (data != "example") {}
if (list === undefined) {}
if (response !== true) {}
if (typeof x === "object" && Object.keys(x).length === 0) {}
if (Array.isArray(x) && x.length === 0) {}
if (value === { a: 1 }) {}
if (value === [1]) {}
if (value === [,]) {}
if (value === {...rest}) {}
if (value < []) {}
if (value + {}) {}
if (value instanceof Object) {}
const empty = {};
if (value === empty) {}
if (value === Object.keys({})) {}
