/* should generate diagnostics */
const a = parseInt(String(number as number), 10);
const b = parseInt(String<() => number>(number), 10);
const c = parseInt(String(number!), 10);
const d = ~~bar!;
const e = (bar as number) | 0;
foo! |= 0;
(foo as number) |= 0;
(foo satisfies number) |= 0;
(<number>foo) |= 0;
(foo as Bar).x |= 0;
foo[key!] |= 0;
