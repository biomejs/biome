(obj?.foo)();
(obj.foo ?? bar?.baz)();
(obj.foo || bar?.baz)();
(obj?.foo && bar)();
(bar && obj?.foo)();
(obj?.foo?.())();
(obj?.foo).bar;
(obj?.foo)[1];
(obj?.foo)`template`
new (obj?.foo)();
new (obj?.foo?.())()
new (obj?.foo?.() || obj?.bar)()
async function foo() { (await obj?.foo)(); }
async function foo() { (await obj?.foo).bar; }
async function foo() { (bar?.baz ?? await obj?.foo)(); }
async function foo() { (bar && await obj?.foo)(); }
async function foo() { (await (bar && obj?.foo))(); }

// spread
[...obj?.foo];
bar(...obj?.foo);
new Bar(...obj?.foo);

// destructuring
const {foo} = obj?.bar;
const {foo} = obj?.bar();
const {foo: bar} = obj?.bar();
const [foo] = obj?.bar;
const [foo] = obj?.bar || obj?.foo;
([foo] = obj?.bar);
const [foo] = obj?.bar?.();
[{ foo } = obj?.bar] = [];
({bar: [ foo ] = obj?.prop} = {});
[[ foo ] = obj?.bar] = [];
async function foo() { const {foo} = await obj?.bar; }
async function foo() { const {foo} = await obj?.bar(); }
async function foo() { const [foo] = await obj?.bar || await obj?.foo; }
async function foo() { ([foo] = await obj?.bar); }

// class declaration
class A extends obj?.foo {}
async function foo() { class A extends (await obj?.foo) {}}

// class expression
var a = class A extends obj?.foo {}
async function foo() { var a = class A extends (await obj?.foo) {}}

// relational operations
foo instanceof obj?.prop
async function foo() { foo instanceof await obj?.prop }
1 in foo?.bar;
async function foo() { 1 in await foo?.bar; }

// for...of
for (foo of obj?.bar);
async function foo() { for (foo of await obj?.bar);}

// sequence expression
(foo, obj?.foo)();
(foo, obj?.foo)[1];
async function foo() { (await (foo,  obj?.foo))(); }
async function foo() { ((foo, await obj?.foo))(); }
async function foo() { (foo, await obj?.foo)[1]; }
async function foo() { (await (foo, obj?.foo)) [1]; }

// conditional expression
(a ? obj?.foo : b)();
(a ? b : obj?.foo)();
(a ? obj?.foo : b)[1];
(a ? b : obj?.foo).bar;
async function foo() { (await (a ? obj?.foo : b))(); }
async function foo() { (a ? await obj?.foo : b)(); }
async function foo() { (await (a ? b : obj?.foo))(); }
async function foo() { (await (a ? obj?.foo : b))[1]; }
async function foo() { (await (a ? b : obj?.foo)).bar; }
async function foo() { (a ? b : await obj?.foo).bar; }

(obj?.foo && obj?.baz).bar

async function foo() { with ( await obj?.foo) {}; }
(foo ? obj?.foo : obj?.bar).bar
