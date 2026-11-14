// A type assertion that isn't used in a way that requires a `prototype`
const asCall = function () {};
(asCall as any)();

// `.name` through a non-null assertion
const nonNullName = function () {};
nonNullName!.name;
