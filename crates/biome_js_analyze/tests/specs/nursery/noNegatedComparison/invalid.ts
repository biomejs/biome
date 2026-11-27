/* should generate diagnostics */
!(foo! === bar);
!((foo as string) === bar);
const foo1 = !(a === b) as boolean;
const foo2 = !(a === b) satisfies boolean;
const foo3 = <boolean>!(a === b);
const foo4 = (!(a === b))!;
