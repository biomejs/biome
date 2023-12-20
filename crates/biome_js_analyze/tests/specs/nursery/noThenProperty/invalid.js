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