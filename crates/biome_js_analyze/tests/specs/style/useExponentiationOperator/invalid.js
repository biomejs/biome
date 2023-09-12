Math.pow(a, b);
(Math).pow(a, b);

// able to catch some workarounds
Math[`pow`](a, b);
(Math)['pow'](a, b);
(Math)["pow"](a, b);
(Math)[`pow`](a, b);

// non-expression parents that don't require parens
var x = Math.pow(a, b);
if(Math.pow(a, b)){}
for(;Math.pow(a, b);){}
switch(foo){ case Math.pow(a, b): break; }
{ foo: Math.pow(a, b) }
function foo(bar, baz = Math.pow(a, b), quux){}
`${Math.pow(a, b)}`

// non-expression parents that do require parens
class C extends Math.pow(a, b) {}

// already parenthesised, shouldn't insert extra parens
+(Math.pow(a, b))
(Math.pow(a, b)).toString()
(class extends (Math.pow(a, b)) {})
class C extends (Math.pow(a, b)) {}

// '**' is right-associative, that applies to both parent and child nodes
a ** Math.pow(b, c);
Math.pow(a, b) ** c;
Math.pow(a, b ** c);
Math.pow(a ** b, c);
a ** Math.pow(b ** c, d ** e) ** f;

// doesn't remove already existing unnecessary parens around the whole expression
(Math.pow(a, b));
foo + (Math.pow(a, b));
(Math.pow(a, b)) + foo;
`${(Math.pow(a, b))}`;

// Optional chaining
Math.pow?.(a, b)
Math?.pow(a, b)
Math?.pow?.(a, b)
;(Math?.pow)(a, b)
;(Math?.pow)?.(a, b)

// doesn't put extra parens
Math.pow((a + b), (c + d))

// tokens that can be adjacent
a+Math.pow(b, c)+d
