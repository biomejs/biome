const foo = {then: 1}
const foo = {["then"]: 1}
const foo = {[`then`]: 1}
const foo = {["then"]() {}}
const foo = {[`then`]() {}}
// const THEN = "then";const foo = {[THEN]() {}}
const foo = {get then() {}}
const foo = {get ["then"]() {}}
const foo = {get [`then`]() {}}
// const THEN = "then";const foo = {get [THEN]() {}}
class Foo {then}
const Foo = class {then}
class Foo {["then"]}
class Foo {[`then`]}
// const THEN = "then";class Foo {[THEN]}
class Foo {then() {}}
class Foo {["then"]() {}}
class Foo {[`then`]() {}}
// const THEN = "then";class Foo {[THEN]() {}}
class Foo {static then}
class Foo {static ["then"]}
class Foo {static [`then`]}
// const THEN = "then";class Foo {static [THEN]}
class Foo {static then() {}}
class Foo {static ["then"]() {}}
class Foo {static [`then`]() {}}
// const THEN = "then";class Foo {static [THEN]() {}}
class Foo {get then() {}}
class Foo {get ["then"]() {}}
class Foo {get [`then`]() {}}
// const THEN = "then";class Foo {get [THEN]() {}}
class Foo {set then(v) {}}
class Foo {set ["then"](v) {}}
class Foo {set [`then`](v) {}}
// const THEN = "then";class Foo {set [THEN](v) {}}
class Foo {static get then() {}}
class Foo {static get ["then"]() {}}
class Foo {static get [`then`]() {}}
// const THEN = "then";class Foo {static get [THEN]() {}}

foo.then = 1
foo["then"] = 1
foo[`then`] = 1
foo.then += 1
foo.then ||= 1
foo.then ??= 1

Object.defineProperty(foo, "then", 1)
Object.defineProperty(foo, `then`, 1)
// const THEN = "then";Object.defineProperty(foo, THEN, 1)
Reflect.defineProperty(foo, "then", 1)
Reflect.defineProperty(foo, `then`, 1)
// const THEN = "then";Reflect.defineProperty(foo, THEN, 1)

Object.fromEntries([["then", 1]])
Object.fromEntries([["then"]])
Object.fromEntries([[`then`, 1]])
// const THEN = "then";Object.fromEntries([[THEN, 1]])
Object.fromEntries([foo, ["then", 1]])

const then = 1; export {then}
const notThen = 1; export {notThen as then}

export const then = 1