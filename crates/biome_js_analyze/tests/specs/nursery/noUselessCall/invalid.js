/* should generate diagnostics */

// `this` is null or undefined
foo.call(undefined, 1, 2);
foo.call(void 0, 1, 2);
foo.call(null, 1, 2);
foo.call((null), 1, 2);
foo.call(undefined);

// `this` is the object of the member expression
obj.foo.call(obj, 1, 2);
a.b.c.foo.call(a.b.c, 1, 2);
a.b(x, y).c.foo.call(a.b(x, y).c, 1, 2);
a[0].call(a, 1, 2);
this.foo.call(this, 1, 2);
class C { #foo() {} bar() { this.#foo.call(this); } }

// apply with an array literal
foo.apply(undefined, [1, 2]);
foo.apply(void 0, [1, 2]);
foo.apply(null, [1, 2]);
foo.apply(null, ([1, 2]));
obj.foo.apply(obj, [1, 2]);
a.b.c.foo.apply(a.b.c, [1, 2]);
a.b(x, y).c.foo.apply(a.b(x, y).c, [1, 2]);
[].concat.apply([ ], [1, 2]);
[].concat.apply([
/*empty*/
], [1, 2]);
abc.get("foo", 0).concat.apply(abc . get("foo",  0 ), [1, 2]);

// optional chaining and parentheses
foo.call?.(undefined, 1, 2);
foo?.call(undefined, 1, 2);
(foo?.call)(undefined, 1, 2);
obj.foo.call?.(obj, 1, 2);
obj?.foo.call(obj, 1, 2);
(obj?.foo).call(obj, 1, 2);
(obj?.foo.call)(obj, 1, 2);
obj?.foo.bar.call(obj?.foo, 1, 2);
(obj?.foo).bar.call(obj?.foo, 1, 2);
obj.foo?.bar.call(obj.foo, 1, 2);
