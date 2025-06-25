/* should not generate diagnostics */

false && Promise.reject("logical operator bypass");
true || Promise.reject("logical operator bypass");
({}) ?? Promise.reject("logical operator bypass");
"one" && 0 && Promise.reject("logical operator bypass");
"" || 1 || Promise.reject("logical operator bypass");
null ?? (() => true) ?? Promise.reject("logical operator bypass");

type Truthy = 1 | "one";
let truthy: Truthy = 1;
truthy || Promise.reject("logical operator bypass");
truthy ?? Promise.reject("logical operator bypass");

type Nullish = null | undefined;
let nullish: Nullish = null;
nullish && Promise.reject("logical operator bypass");

let falsy: Nullish | false;
falsy && Promise.reject("logical operator bypass");

interface Foo {}
let foo: Foo;
foo ?? Promise.reject("logical operator bypass");

class C {}
let c = new C;
c || Promise.reject("logical operator bypass");
c ?? Promise.reject("logical operator bypass");

let o: object;
o || Promise.reject("logical operator bypass");
o ?? Promise.reject("logical operator bypass");
