// should generate diagnostics

foo.apply(null, args);
foo.apply(null, [1, 2, 3]);
foo.apply(undefined, args);
obj.foo.apply(obj, args);
