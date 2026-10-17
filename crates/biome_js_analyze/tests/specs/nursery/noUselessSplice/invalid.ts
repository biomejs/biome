/* should generate diagnostics */
array.splice(0 as const, 1 as const);
array.splice(0 satisfies number, 1 satisfies number);
array.splice(<number>0, <number>0, element);
array.splice(array.length - 1!, 1!);
array.splice(0!, array.length);
array.splice(0, array.length as number);
array.splice((array.length as number) - 1, 1);
(array as string[]).splice(0);
(array satisfies string[]).splice((array satisfies string[]).length, 0, element);
function f(foo: number[]) { foo.splice(); }
