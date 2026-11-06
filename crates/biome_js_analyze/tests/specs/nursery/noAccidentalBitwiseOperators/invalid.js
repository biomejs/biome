/* should generate diagnostics */

// `&` used to guard a property access
obj & obj.a;
if (obj & obj.prop) {}
obj & obj[key];
(obj) & obj.a;
obj & (obj.a);
obj /* comment */ & obj.a;

// `|` used as a fallback
options | {};
options | '';
options | true;
options | [];
options | `template`;
x | function () {};
x | (() => {});
x | class {};
foo() | {};
a.b | {};
a | b | {};

// `|=` used as a fallback
input |= '';
input |= {};
input |= false;

// The fix keeps the meaning of the surrounding expression
obj & obj.a | x;
x | obj & obj.a;
x ?? obj & obj.a;
a | {} && b;
a | {} || b;
b || a | {};
b && a | {};
x | /* before */ obj & obj.a /* after */;
