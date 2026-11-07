/* should generate diagnostics */
array.splice(index, 1 as const, element);
array.splice(index, <number>1, element);
array.splice(index, 1!, element);
array.splice(index, 1 satisfies number, element);
array.splice(-1 as const, 0 as const, element);
array.splice(-1 satisfies number, 0 satisfies number, element);
array.toSpliced(index, 1 as const, element);
array.toSpliced(-1 satisfies number, 1, element);
array.toSpliced(-(1 satisfies number), 1, element);
array.toSpliced(-1 as const, 0 as const, element);
array.splice(`1` as const, 1, element);
array.splice(index as number, 1, element);
array.splice(null as any, 1, element);
array.splice(undefined!, 1, element);
(array as unknown[]).splice(0, 1, element);
function f(foo: unknown[]) { foo.splice(index, 1, element); }
