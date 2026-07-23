/* should not generate diagnostics */

// The `this` argument is a specific object, which is not the same as the callee's object.
foo.apply(obj, [1, 2, 3]);
foo.apply(obj, args);
obj.foo.apply(bar, args);

// The number of arguments is not 2.
foo.apply(null);
foo.apply();
