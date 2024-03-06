// `object`
const then = {}
const notThen = then
const then = then.then
const foo = {notThen: 1}
const foo = {notThen() {}}
const foo = {[then]: 1}
function foo({then}) {}

// `class`
class then {}
class Foo {notThen}
class Foo {notThen() {}}
class Foo {[then]}
class Foo {#then}
class Foo {#then() {}}
class Foo {[then]() {}}
class Foo {get notThen() {}}
class Foo {get #then() {}}
class Foo {get [then]() {}}
class Foo {static notThen}
class Foo {static notThen() {}}
class Foo {static #then}
class Foo {static #then() {}}
class Foo {static [then]}
class Foo {static [then]() {}}
class Foo {static get notThen() {}}
class Foo {static get #then() {}}
class Foo {static get [then]() {}}
class Foo {notThen = then}

// Assign
foo[then] = 1
foo.notThen = 1
then.notThen = then.then
const NOT_THEN = "no-then";foo[NOT_THEN] = 1
foo.then ++
++ foo.then
delete foo.then
typeof foo.then
foo.then != 1

// `{Object,Reflect}.defineProperty`
Object.defineProperty(foo, then, 1)
Object.defineProperty(foo, "not-then", 1)
Reflect.defineProperty(foo, then, 1)
Reflect.defineProperty(foo, "not-then", 1)
Object.defineProperty(foo, "then", )
Object.defineProperty(...foo, "then", 1)
Object.defineProperty(foo, ...["then", 1])

// `Object.fromEntries`
Object.fromEntries([then, 1])
Object.fromEntries([,,])
Object.fromEntries([[,,],[]])
Object.fromEntries([[["then", 1]]])
NotObject.fromEntries([["then", 1]])
Object.notFromEntries([["then", 1]])
Object.fromEntries?.([["then", 1]])
Object?.fromEntries([["then", 1]])
Object.fromEntries([[..."then", 1]])
Object.fromEntries([["then", 1]], extraArgument)
Object.fromEntries(...[["then", 1]])

// `export`
export {default} from "then"
const then = 1; export {then as notThen}
export default then
export function notThen(){}
export class notThen {}
export default function then (){}
export default class then {}
export default function (){}
export default class {}

// `export variables`
export const notThen = 1
export const {then: notThen} = 1
export const {then: notThen = then} = 1