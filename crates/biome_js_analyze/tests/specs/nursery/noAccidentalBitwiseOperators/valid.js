/* should not generate diagnostics */

// Bitwise math
a | b;
a & b;
flags & MASK;
x | 0;
x | 1;
options | someVariable;
a ^ b;
a << b;
a >> b;
a >>> b;
~x;
x | (a + b);
foo() | bar();
x | null;
x | 1n;
x | /regex/;
x | tag`template`;

// Logical operators
a && b;
a || b;
obj && obj.prop;

// `&` only reports a variable followed by a property of the same variable
obj1 & obj2.a;
obj.a & obj.b;
obj & obj.a.b;
a & b.c;
this & this.a;
obj & obj;
obj & obj.prop();
obj & obj?.a;
obj & obj?.[key];

// `&=` is not reported
x &= {};
x &= 1;

// `|=` with a value that may be a number
x |= 1;
x |= y;
