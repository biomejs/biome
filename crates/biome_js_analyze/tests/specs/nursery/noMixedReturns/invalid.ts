/* should generate diagnostics */
function foo1(a: boolean): number | undefined {
    if (a) return 1;
    return;
}

class A {
    @decorator
    method(a: boolean): number | undefined {
        if (a) return 1;
    }
}
