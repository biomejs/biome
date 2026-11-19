/* should not generate diagnostics */

// Under the default `infer` compilation mode, functions that don't follow
// React naming conventions are not analyzed, so idiomatic module-level
// mutable state in utility code is not flagged.

let counter = 0;

export function increment() {
    counter = counter + 1;
    return counter;
}

export function stamp(record) {
    record.time = Date.now();
    return record;
}
