/* should generate diagnostics */

const a = 1;
const b = 2;

Math.min(foo(), b) < c ? Math.min(foo(), b) : c;
Math.min(...values) < c ? Math.min(...values) : c;
Math.min([...values], b) < c ? Math.min([...values], b) : c;
Math.min(tag`value`, b) < c ? Math.min(tag`value`, b) : c;
Math["min"](a, b) < c ? Math["min"](a, b) : c;
Math?.min(a, b) < c ? Math?.min(a, b) : c;
Math.min?.(a, b) < c ? Math.min?.(a, b) : c;
Math.min(a, b) < c ? Math.min(/* keep */ a, b) : c;
Math.min(a, b) /* condition */ < c ? /* branch */ Math.min(a, b) : c;
Math.min(a, b) < c
    ? Math.min(a, b) // keep branch
    : c;
Math.min(a, b) // keep condition
    < c ? Math.min(a, b) : c;

let conversions = 0;
const object = { valueOf() { return ++conversions; } };
Math.min(object, 10) < 20 ? Math.min(object, 10) : 20;
Math.max(object, 10) > 0 ? Math.max(object, 10) : 0;
Math.min(Math.max(object, 10), 20) < 30 ? Math.min(Math.max(object, 10), 20) : 30;

const alias = object;
Math.min(alias, 10) < 20 ? Math.min(alias, 10) : 20;

const primitiveHook = { [Symbol.toPrimitive]() { return ++conversions; } };
Math.min(primitiveHook, 10) < 20 ? Math.min(primitiveHook, 10) : 20;

function unknownArgument(value) {
    return Math.min(value, 10) < 20 ? Math.min(value, 10) : 20;
}

let mutable = 1;
mutable = object;
Math.min(mutable, 10) < 20 ? Math.min(mutable, 10) : 20;

const forward = later;
const later = forward;
Math.min(forward, 10) < 20 ? Math.min(forward, 10) : 20;

function shadowedUndefined(undefined) {
    return Math.min(undefined, 10) < 20 ? Math.min(undefined, 10) : 20;
}
