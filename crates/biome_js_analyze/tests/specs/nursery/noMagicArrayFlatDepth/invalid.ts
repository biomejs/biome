/* should generate diagnostics */
function f(foo: number[][]) { foo.flat(2); }
array.flat(-2 as number);
array.flat(2 satisfies number);
array.flat(<number>2);
array.flat(2!);
