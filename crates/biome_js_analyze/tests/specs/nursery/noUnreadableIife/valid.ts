/* should not generate diagnostics */
const foo1 = (bar => bar as number)();
const foo2 = (<T,>(bar: T) => bar)(1);
