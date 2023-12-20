const then = {}
const notThen = then
const then = then.then
const foo = {notThen: 1}
const foo = {notThen() {}}
const foo = {[then]: 1}
// const NOT_THEN = "no-then";const foo = {[NOT_THEN]: 1}
function foo({then}) {}

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

foo[then] = 1
foo.notThen = 1
then.notThen = then.then
const NOT_THEN = "no-then";foo[NOT_THEN] = 1
foo.then ++
++ foo.then
delete foo.then
typeof foo.then
foo.then != 1

Object.defineProperty(foo, then, 1)
Object.defineProperty(foo, "not-then", 1)
// const then = "no-then";Object.defineProperty(foo, then, 1)
Reflect.defineProperty(foo, then, 1)
Reflect.defineProperty(foo, "not-then", 1)
// const then = "no-then";Reflect.defineProperty(foo, then, 1)
Object.defineProperty(foo, "then", )
Object.defineProperty(...foo, "then", 1)
Object.defineProperty(foo, ...["then", 1])