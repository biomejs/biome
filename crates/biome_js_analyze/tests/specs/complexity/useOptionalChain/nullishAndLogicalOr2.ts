(foo1?.foo2 ?? {}).#foo3;
((() => foo())() ?? {}).#bar;
const foo = (bar ?? {}).#baz;
(foo.bar ?? {})[baz];
((foo1 ?? {}).#foo2 ?? {}).#foo3;

(foo ?? undefined ?? {}).#bar;
(foo() ?? bar ?? {}).#baz;
((foo1 ? foo2 : foo3) ?? {}).#foo4;

if (foo) { (foo ?? {}).#bar; }
if ((foo ?? {}).#bar) { foo.bar; }

(undefined && foo ?? {}).#bar;
(a > b || {}).#bar;
(((typeof x) as string) || {}).#bar;

(void foo() || {}).#bar;
((a ? b : c) || {}).#bar;

((a instanceof Error) || {}).#bar;
((a << b) || {}).#bar;
((foo ** 2) || {}).#bar;
(foo ** 2 || {}).#bar;
(foo++ || {}).#bar;
(+foo || {}).#bar;


//this expression
(this || {}).bar;
(this || ({})).bar;
(await this || {}).bar;
const foo = (this || {}).baz;

((this || {}).foo2 || {}).foo3;

if (foo) { (this || {}).bar; }
if ((this || {}).bar) { foo.bar; }

(undefined && this || {}).bar;
(this ?? {}).bar;
(this ?? ({})).bar;
(await this ?? {}).bar;

const foo = (this ?? {}).baz;
((this ?? {}).foo2 ?? {}).foo3;

if (foo) { (this ?? {}).bar; }
if ((this ?? {}).bar) { foo.bar; }

(undefined && this ?? {}).bar;
(((typeof this) as string) || {}).bar;

// this expression with private name
(this || {}).#bar;
(this || ({})).#bar;
(await this || {}).#bar;
const foo = (this || {}).#baz;

((this || {}).#foo2 || {}).#foo3;

if (foo) { (this || {}).#bar; }
if ((this || {}).#bar) { foo.bar; }

(undefined && this || {}).#bar;
(this ?? {}).#bar;
(this ?? ({})).#bar;
(await this ?? {}).#bar;

const foo = (this ?? {}).#baz;
((this ?? {}).#foo2 ?? {}).#foo3;

if (foo) { (this ?? {}).#bar; }
if ((this ?? {}).#bar) { foo.bar; }

(undefined && this ?? {}).#bar;
(((typeof this) as string) || {}).#bar;
// (new foo || {}).bar;  // tracked here https://github.com/rome/tools/issues/3257
(foo() || {}).bar;
((foo || {}).bar() || {}).baz;