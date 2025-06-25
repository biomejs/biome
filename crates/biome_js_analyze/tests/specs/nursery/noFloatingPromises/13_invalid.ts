true && Promise.reject("logical operator bypass");
false || Promise.reject("logical operator bypass");
null ?? Promise.reject("logical operator bypass");
"one" && 1 && Promise.reject("logical operator bypass");
"" || 0 || Promise.reject("logical operator bypass");
null ?? undefined ?? Promise.reject("logical operator bypass");

type Truthy = 1 | "one";
let truthy: Truthy = 1;
truthy && Promise.reject("logical operator bypass");

type Nullish = null | undefined;
let nullish: Nullish = null;
nullish || Promise.reject("logical operator bypass");
nullish ?? Promise.reject("logical operator bypass");

let either: Truthy | Nullish = 1;
either && Promise.reject("logical operator bypass");
either || Promise.reject("logical operator bypass");
either ?? Promise.reject("logical operator bypass");

interface Foo {}
let foo: Foo;
foo && Promise.reject("logical operator bypass");
foo || Promise.reject("logical operator bypass");

class C {}
let c = new C;
c && Promise.reject("logical operator bypass");

let o: object;
o && Promise.reject("logical operator bypass");

type Params = {
    option: boolean | Nullish;
};

function functionWithParams({ option }: Params) {
    option ?? Promise.reject("logical operator bypass");
}
