/* should generate diagnostics */

// A `void` function does not need a trailing `return;`, even when another code
// path returns a value.
export function explicitVoid(flag: boolean): void {
    if (flag) {
        console.log(flag);
    }
    return;
}

// The return type is inferred as `void`, because no `return` statement of the
// function returns a value.
export function inferredVoid() {
    console.log("side effect");
    return;
}

// A value returned by a nested function does not contribute to the return type
// of the enclosing function.
export function inferredVoidWithNestedFunction() {
    const nested = () => {
        return 1;
    };
    nested();
    return;
}

// `any` can always be satisfied by an implicit fallthrough.
export function explicitAny(flag: boolean): any {
    if (flag) {
        return 1;
    }
    return;
}

// An async function without a value return is typed `Promise<void>`.
export async function inferredAsyncVoid() {
    await Promise.resolve();
    return;
}
