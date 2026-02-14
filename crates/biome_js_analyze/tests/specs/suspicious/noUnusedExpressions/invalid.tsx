/* should generate diagnostics */
// Source: https://github.com/eslint/eslint/blob/main/tests/lib/rules/no-unused-expressions.js
// Copyright OpenJS Foundation and other contributors, MIT license

0;

a;

f(), 0;

{0};

[];

a && b();

a() || false;

a || (b = c);

a ? b() || (c = d) : e;

`untagged template literal`

tag`tagged template literal`

a ? b() : c();

foo.bar;

!a;

+a;

"not a directive";

function foo() {"directive one"; f(); "directive two"; }

if (0) { "not a directive"; f(); }

function foo() { var foo = true; "use strict"; }

var foo = () => { var foo = true; "use strict"; }

obj?.foo;

obj?.foo.bar;

obj?.foo().bar;

<div />;

<></>;

class C { static { 'use strict'; } }

class C {
    static {
        'foo'
        'bar'
    }
}
