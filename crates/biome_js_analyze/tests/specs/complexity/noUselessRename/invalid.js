let { /*before*/ foo: foo /*after*/ } = obj;

let { a, foo: foo } = obj;

let { foo: foo, b } = obj;

let {
	foo: { bar: bar },
} = obj;

let { /*before*/ foo: foo /*after*/ = /*before default*/ a /*after default*/ } =
	obj;

function f({ foo: foo }) {}

({ foo: foo }) => {};

import { /*before*/ foo as foo /*after*/ } from "foo";

import { a as a } from "foo";

export { /*before*/ foo as foo /*after*/ };

export { /*before*/ foo as foo /*after*/ } from "foo";

// following cases are supported by ESLint

//import {a as \u0061} from 'foo';
//import {\u0061 as a} from 'foo';
//export {\u0061 as a};
//export {a as \u0061};

//let {"a": a} = obj;
//import { "a" as a}
//export { a as "a"}
