/* should generate diagnostics */
const foo1 = ((bar: number) => (bar as number))(1);
const foo2 = (<T,>(bar: T): T => (bar))(1);
