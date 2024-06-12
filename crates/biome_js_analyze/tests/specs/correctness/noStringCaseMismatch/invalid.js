s.toUpperCase() === 'abc';
s.toUpperCase() == 'abc';
'abc' === s.toUpperCase();
s.toLowerCase() === "\u001aX";
s.toLowerCase() === "\u{001a}X";
s.toLowerCase() === "\xaaX";
s.toLowerCase() === "\nX";

if (s.toUpperCase() === 'abc' && c == d && e == f) {};
while (s.toUpperCase() === 'abc' && c == d && e == f) {};
while (s.toUpperCase() === 'abc') {};
let b = s.toLowerCase() === `eFg`;;
do {} while (s.toLowerCase() === 'ABC');;
for (; s.toLowerCase() === 'ABC'; ) {};

switch (s.toUpperCase()) { case 'ABC': case 'abc': case 'aBc': default: }

for (; s['toLowerCase']() === 'ABC'; ) {}
for (; s[`toUpperCase`]() === 'abc'; ) {}

switch (s['toLowerCase']()) { case 'Abc': case 'aBc': case 'abC': default: }
