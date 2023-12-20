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
// const THEN = "then";class Foo {[THEN]}'
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