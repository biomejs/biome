/* should generate diagnostics */

// trailing return in a TypeScript function where no path returns a value
export function a(): void {
    doSomething();
    return;
}

// a valued return in a nested function does not protect the outer return:
// the outer trailing return is still useless
export function b(): void {
    function inner(): number {
        return 1;
    }
    inner();
    return;
}
