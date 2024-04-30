new Object()
new Array()
new ArrayBuffer()
new BigInt64Array()
new BigUint64Array()
new DataView()
new Date()
new Error()
new Float32Array()
new Float64Array()
new Function()
new Int8Array()
new Int16Array()
new Int32Array()
new Map()
new WeakMap()
new Set()
new WeakSet()
new Promise()
new RegExp()
new Uint8Array()
new Uint16Array()
new Uint32Array()
new Uint8ClampedArray()
new SharedArrayBuffer()
new Proxy()
new WeakRef()
new FinalizationRegistry()
new window.Object({})
new globalThis.Object()
function foo() {
    return new globalThis.Object()
}

Boolean()
BigInt()
Number()
String()
Symbol()
window.String()
globalThis.String(123)
function foo() {
    return globalThis.String()
}

function varCheck() {
    {
        var String = class {}
    }
    // This should not be reported
    return new String()
}