// should generate diagnostics

foo.call(undefined, 1, 2, 3);
foo.call(null, 1, 2, 3);
foo.apply(undefined, [1, 2, 3]);
foo.apply(null, [1, 2, 3]);

obj.foo.call(obj, 1, 2, 3);
obj.foo.apply(obj, [1, 2, 3]);
