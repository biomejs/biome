// `object`
const foo = {then: 1}
const foo = {["then"]: 1}
const foo = {[`then`]: 1}
const foo = {["then"]() {}}
const foo = {[`then`]() {}}
const foo = {get then() {}}
const foo = {get ["then"]() {}}
const foo = {get [`then`]() {}}

// `class`
class Foo {then}
const Foo = class {then}
class Foo {["then"]}
class Foo {[`then`]}
class Foo {then() {}}
class Foo {["then"]() {}}
class Foo {[`then`]() {}}
class Foo {static then}
class Foo {static ["then"]}
class Foo {static [`then`]}
class Foo {static then() {}}
class Foo {static ["then"]() {}}
class Foo {static [`then`]() {}}
class Foo {get then() {}}
class Foo {get ["then"]() {}}
class Foo {get [`then`]() {}}
class Foo {set then(v) {}}
class Foo {set ["then"](v) {}}
class Foo {set [`then`](v) {}}
class Foo {static get then() {}}
class Foo {static get ["then"]() {}}
class Foo {static get [`then`]() {}}

// Assign
foo.then = 1
foo["then"] = 1
foo[`then`] = 1
foo.then += 1
foo.then ||= 1
foo.then ??= 1

// `{Object,Reflect}.defineProperty`
Object.defineProperty(foo, "then", 1)
Object.defineProperty(foo, `then`, 1)
Reflect.defineProperty(foo, "then", 1)
Reflect.defineProperty(foo, `then`, 1)

// `Object.fromEntries`
Object.fromEntries([["then", 1]])
Object.fromEntries([["then"]])
Object.fromEntries([[`then`, 1]])
Object.fromEntries([foo, ["then", 1]])

// `export`
const then = 1; export {then}
const notThen = 1; export {notThen as then}

// `export variables`
export const then = 1