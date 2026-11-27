/* should not generate diagnostics */
a === b;
a !== b;
a > b;
!Array.isArray(value);
!(a === b && c === d);
!(a || b);
!(key in object);
!(value instanceof Class);
!foo;
!!(a === b);
!(!(a === b));
!foo === bar;
!(a > b);
!(a >= b);
!(a < b);
!(a <= b);
!(a?.b > c);
!(a + b);
!((a === b) && c);
typeof (a === b);
if (!(chr >= 0)) {}
async function foo() { return await !(a > b); }
const foo = a + !(b > c);
