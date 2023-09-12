(foo || {}).bar;
(foo || ({})).bar;
(await foo || {}).bar;
(foo1?.foo2 || {}).foo3;
((() => foo())() || {}).bar;
const foo = (bar || {}).baz;
(foo.bar || {})[baz];

((foo1 || {}).foo2 || {}).foo3;
(foo || undefined || {}).bar;

(foo() || bar || {}).baz;
((foo1 ? foo2 : foo3) || {}).foo4;

if (foo) { (foo || {}).bar; }
if ((foo || {}).bar) { foo.bar; }

(undefined && foo || {}).bar;
(foo ?? {}).bar;
(foo ?? ({})).bar;
(await foo ?? {}).bar;

(foo1?.foo2 ?? {}).foo3;
((() => foo())() ?? {}).bar;
const foo = (bar ?? {}).baz;
(foo.bar ?? {})[baz];
((foo1 ?? {}).foo2 ?? {}).foo3;

(foo ?? undefined ?? {}).bar;
(foo() ?? bar ?? {}).baz;
((foo1 ? foo2 : foo3) ?? {}).foo4;

if (foo) { (foo ?? {}).bar; }
if ((foo ?? {}).bar) { foo.bar; }

(undefined && foo ?? {}).bar;
(a > b || {}).bar;
(((typeof x) as string) || {}).bar;

(void foo() || {}).bar;
((a ? b : c) || {}).bar;

((a instanceof Error) || {}).bar;
((a << b) || {}).bar;
((foo ** 2) || {}).bar;
(foo ** 2 || {}).bar;
(foo++ || {}).bar;
(+foo || {}).bar;

// private name
(foo || {}).#bar;
(foo || ({})).#bar;
(await foo || {}).#bar;
(foo1?.foo2 || {}).#foo3;
((() => foo())() || {}).#bar;
const foo = (bar || {}).#baz;
(foo.bar || {})[baz];

((foo1 || {}).#foo2 || {}).#foo3;
(foo || undefined || {}).#bar;

(foo() || bar || {}).#baz;
((foo1 ? foo2 : foo3) || {}).#foo4;

if (foo) { (foo || {}).#bar; }
if ((foo || {}).#bar) { foo.bar; }

(undefined && foo || {}).#bar;
(foo ?? {}).#bar;
(foo ?? ({})).#bar;
(await foo ?? {}).#bar;
