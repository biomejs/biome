ArrayBuffer()
BigInt64Array()
BigUint64Array()
DataView()
Float32Array()
Float64Array()
Int16Array()
Int32Array()
Int8Array()
Map()
Proxy()
Set()
SharedArrayBuffer()
Uint16Array()
Uint32Array()
Uint8Array()
Uint8ClampedArray()
WeakMap()
WeakSet()
WeakRef()
FinalizationRegistry()
window.Map()
globalThis.Map()
function foo() {
    return /** Start */ globalThis.Map([]) /** End */
}

new BigInt(123)
new Symbol()
new window.BigInt(123)
new globalThis.BigInt(123)
function foo() {
    return /** Start */ new globalThis.BigInt(123) /** End */
}