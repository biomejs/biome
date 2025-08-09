[] && greeting;
[] || greeting;
[] ?? greeting;
[] == true;
true == [];
[] != true;
[] === true;
[] !== true;

!foo == null;
!foo ?? bar;
(a + b) / 2 ?? bar;
String(foo.bar) ?? baz;
"hello" + name ?? "";
[foo?.bar ?? ""] ?? [];

// Logical expression with constant truthiness
true && hello;
true || hello;
true && foo;
'' && foo;
100 && foo;
+100 && foo;
-100 && foo;
~100 && foo;
/[a-z]/ && foo;
Boolean([]) && foo;
Boolean() && foo;
Boolean([], n) && foo;
({}) && foo;
[] && foo;
(() => {}) && foo;
(function() {}) && foo;
(class {}) && foo;
(class { valueOf() { return x; } }) && foo;
(class { [x]() { return x; } }) && foo;
new Foo() && foo;

// (boxed values are always truthy)
new Boolean(unknown) && foo;
(bar = false) && foo;
(bar.baz = false) && foo;
(bar[0] = false) && foo;
`hello ${hello}` && foo;
void bar && foo;
!true && foo;
typeof bar && foo;
(bar, baz, true) && foo;
undefined && foo;

// Logical expression with constant nullishness
({}) ?? foo;
([]) ?? foo;
(() => {}) ?? foo;
(function() {}) ?? foo;
(class {}) ?? foo;
new Foo() ?? foo;
1 ?? foo;
/[a-z]/ ?? foo;
`${''}` ?? foo;
(a = true) ?? foo;
(a += 1) ?? foo;
(a -= 1) ?? foo;
(a *= 1) ?? foo;
(a /= 1) ?? foo;
(a %= 1) ?? foo;
(a <<= 1) ?? foo;
(a >>= 1) ?? foo;
(a >>>= 1) ?? foo;
(a |= 1) ?? foo;
(a ^= 1) ?? foo;
(a &= 1) ?? foo;
undefined ?? foo;
!bar ?? foo;
void bar ?? foo;
typeof bar ?? foo;
+bar ?? foo;
-bar ?? foo;
~bar ?? foo;
++bar ?? foo;
bar++ ?? foo;
--bar ?? foo;
bar-- ?? foo;
(x == y) ?? foo;
(x + y) ?? foo;
(x / y) ?? foo;
(x instanceof String) ?? foo;
(x in y) ?? foo;
Boolean(x) ?? foo;
String(x) ?? foo;
Number(x) ?? foo;

// Binary expression with comparison to null
({}) != null;
({}) == null;
null == ({});
({}) == undefined;
undefined == ({});

// Binary expression with loose comparison to boolean
({}) != true;
({}) == true;
([]) == true;
([a, b]) == true;
(() => {}) == true;
(function() {}) == true;
void foo == true;
typeof foo == true;
![] == true;
true == class {};
true == 1;
undefined == true;
true == undefined;
`hello` == true;
/[a-z]/ == true;
({}) == Boolean({});
({}) == Boolean();
({}) == Boolean(() => {}, foo);

// Binary expression with strict comparison to boolean
({}) !== true;
({}) == !({});
({}) === true;
([]) === true;
(function() {}) === true;
(() => {}) === true;
!{} === true;
typeof n === true;
void n === true;
+n === true;
-n === true;
~n === true;
true === true;
1 === true;
'hello' === true;
/[a-z]/ === true;
undefined === true;
(a = {}) === true;
(a += 1) === true;
(a -= 1) === true;
(a *= 1) === true;
(a %= 1) === true;
(a ** b) === true;
(a << b) === true;
(a >> b) === true;
(a >>> b) === true;
--a === true;
a-- === true;
++a === true;
a++ === true;
(a + b) === true;
(a - b) === true;
(a * b) === true;
(a / b) === true;
(a % b) === true;
(a | b) === true;
(a ^ b) === true;
(a & b) === true;
Boolean(0) === Boolean(1);
true === String(x);
true === Number(x);
Boolean(0) == !({});

// Binary expression with strict comparison to null
({}) !== null;
({}) === null;
([]) === null;
(() => {}) === null;
(function() {}) === null;
(class {}) === null;
new Foo() === null;
`` === null;
1 === null;
'hello' === null;
/[a-z]/ === null;
true === null;
null === null;
a++ === null;
++a === null;
--a === null;
a-- === null;
!a === null;
typeof a === null;
delete a.b === null;
void a === null;
undefined === null;
(x = {}) === null;
(x += y) === null;
(x -= y) === null;
(a, b, {}) === null;

// Binary expression with strict comparison to undefined
({}) !== undefined;
({}) === undefined;
([]) === undefined;
(() => {}) === undefined;
(function() {}) === undefined;
(class {}) === undefined;
new Foo() === undefined;
`` === undefined;
1 === undefined;
'hello' === undefined;
/[a-z]/ === undefined;
true === undefined;
null === undefined;
a++ === undefined;
++a === undefined;
--a === undefined;
a-- === undefined;
!a === undefined;
typeof a === undefined;
delete a.b === undefined;
void a === undefined;
undefined === undefined;
(x = {}) === undefined;
(x += y) === undefined;
(x -= y) === undefined;
(a, b, {}) === undefined;

/*
 * If both sides are newly constructed objects, we can tell they will
 * never be equal, even with == equality.
 */
[a] == [a];
[a] != [a];
({}) == [];

// Comparing to always new objects
x === {};
x !== {};
x === [];
x === (() => {});
x === (function() {});
x === (class {});
x === new Boolean();
x === new Promise();
x === new WeakSet();
x === (foo, {});
x === (y = {});
x === (y ? {} : []);
x === /[a-z]/;

// It's not obvious what this does, but it compares the old value of `x` to the new object.
x === (x = {});

window.abc && false && anything;
window.abc || true || anything;
window.abc ?? 'non-nullish' ?? anything;
