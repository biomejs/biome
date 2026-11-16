/* should generate diagnostics */

foo ? (bar ? baz as string : fallback) : fallback;
foo ? (bar ? baz : object?.property!) : object?.property!;
foo ? (bar as boolean ? fallback : value) : fallback;
outer ? (inner ? (value as /* keep */ string) : fallback) : fallback;
x === 1 ? a : ((x as number) === 2 ? b : (x! === 3 ? a : c));
