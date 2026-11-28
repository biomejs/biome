/* should generate diagnostics */
(foo as string[]).slice((foo as string[]).length - 1);
(array as number[]).splice((array as number[]).length - 2, 1);
Array.prototype.slice.call(foo as any[], (foo as any[]).length - 1);
foo!.slice(foo!.length - 1);
(foo satisfies string[]).slice((foo satisfies string[]).length - 1);
foo.slice((foo.length satisfies number) - 1);
foo.slice((foo.length - 1) as number);
foo.slice(foo.length - (1 as const));
foo.slice(<number>foo.length - 1);
array!.splice(array!.length - 2, 1);
