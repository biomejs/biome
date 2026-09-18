/* should generate diagnostics */
global;
self;
window;

window.foo;
window.foo();

window > 10;
10 > window;
window ?? 10;
10 ?? window;
window.foo = 123;
window = 123;
obj.a = window;

function* gen() { yield window };
async function gen() { await window };

window ? foo : bar;
foo ? window : bar;
foo ? bar : window;

function foo() { return window }
new window();

const obj = { foo: window.foo, bar: window.bar, window: window }

function sequenceTest() { let x, y; x = (y = 10, y + 5, window); console.log(x, y); }

window`Hello ${42} World`
`Hello ${window.foo} World`
var str = `Hello ${window.foo} World`

delete window.foo

++window
++window.foo

for (var attr in window) { }
for (window.foo = 0; i < 10; window.foo++) { }
for (const item of window.foo) { }
for (const item of window) { }

switch (window) {}
switch (true) { case window: break; }
switch (true) { case window.foo: break; }
while (window) { }
do {} while (window) {}
if (window) {}

throw window

var foo = window

function foo (name = window) { }

self.innerWidth;

self.innerHeight;

window.crypto;
window.addEventListener("play", () => {})
window.onplay = function () {}
function greet({ name = window.foo }) {}
({ foo: window.foo } = {})
[window.foo] = []
foo[window]
foo[window.foo]

typeof window !== "undefined";
typeof self !== "undefined";
typeof global !== "undefined";
typeof window.something === "function";
typeof self.something === "function";
typeof global.something === "function";

global.global_did_not_declare_in_language_options;
window.window_did_not_declare_in_language_options;
self.self_did_not_declare_in_language_options;
