// should not generate diagnostics

foo.call(obj, 1, 2, 3);
foo.apply(obj, [1, 2, 3]);

obj.foo.call(null, 1, 2, 3);
obj.foo.apply(null, [1, 2, 3]);

obj.foo.call(otherObj, 1, 2, 3);
obj.foo.apply(otherObj, [1, 2, 3]);

foo.apply(undefined, args);
foo.apply(null, args);
obj.foo.apply(obj, args);

foo.call();
foo.apply();

obj["foo"].call(obj, 1, 2);
obj.foo.call(obj.foo, 1, 2);
a[i++].foo.call(a[i++], 1, 2, 3);
a[++i].foo.call(a[i], 1, 2, 3);
