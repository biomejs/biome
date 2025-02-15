arbitraryFunctΩion(n) ?? foo;
foo.Boolean(n) ?? foo;
(x += 1) && foo;
`${bar}` && foo;
bar && foo;
delete bar.baz && foo;
true ? foo : bar; // We leave ConditionalExpression for `noConstantCondition`.
new Foo() == true;
foo == true;
`${foo}` == true;
`${foo}${bar}` == true;
`0${foo}` == true;
`00000000${foo}` == true;
`0${foo}.000` == true;
[n] == true;

delete bar.baz === true;

foo.Boolean(true) && foo;
(() => { function Boolean(n) { return n; }; Boolean(x) ?? foo; })();
(() => { function String(n) { return n; }; String(x) ?? foo; })();
(() => { function Number(n) { return n; }; Number(x) ?? foo; })();
(() => { function Boolean(n) { return Math.random(); }; Boolean(x) === 1; })();
(() => { function Boolean(n) { return Math.random(); }; Boolean(1) == true; })();

new Foo() === x;
x === new someObj.Promise();
Boolean(foo) === true;
function foo(undefined) { undefined ?? bar; };
function foo(undefined) { undefined == true; };
function foo(undefined) { undefined === true; };
[...arr, 1] == true;
[,,,] == true;
new Foo() === bar;
(foo && true) ?? bar;
foo ?? null ?? bar;
a ?? (doSomething(), undefined) ?? b;
a ?? (something = null) ?? ;
