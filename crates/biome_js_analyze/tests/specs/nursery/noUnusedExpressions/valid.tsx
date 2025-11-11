/* should not generate diagnostics */
// Source: https://github.com/eslint/eslint/blob/main/tests/lib/rules/no-unused-expressions.js
// Copyright OpenJS Foundation and other contributors, MIT license

"use strict";
function f(){}
a = b;
new a;
{};
f(); g()
i++;
a();
delete foo.bar;
void new C;
function foo() {"use strict"; return true; }
var foo = () => {"use strict"; return true; }
function foo() {"directive one"; "directive two"; f(); }
function foo() { var foo = "use strict"; return true; }
function* foo(){ yield 0; }
async function foo() { await 5; }
async function foo() { await foo.bar; }
import("foo");
func?.("foo");
obj?.foo("bar")
var partial = <div />;
var partial = <></>;
