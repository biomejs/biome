Object()
Array()
Date()
Error()
Function()
Promise()
RegExp()
window.Object({})
globalThis.Object()
function foo() {
    return /** Start */ globalThis.Object({ foo: 'bar' }) /** End */
}

new Boolean()
new Number()
new String()
new window.String(123)
new globalThis.String()
function foo() {
    return /** Start */ new globalThis.String("foo") /** End */
}