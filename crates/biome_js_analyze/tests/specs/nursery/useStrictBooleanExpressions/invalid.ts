// should generate diagnostics
function conditions(number: number | null, string: string | undefined, boolean?: boolean) {
    if (number) {}
    while (string) {}
    do {} while (boolean);
    for (; number;) {}
    const conditional = string ? 1 : 2;
    !boolean;
    !!number;
    const and = number && 1;
    const or = string || "fallback";
    if (true && number) {}
    if (false || string) {}
    if (((boolean))) {}
    if (number && string && boolean) {}
    if (number ?? string) {}
}
function unrestricted(any: any, unknown: unknown) {
    if (any) {}
    if (unknown) {}
}
function generic<T>(value: T) {
    if (value) {}
}
function mixed(value: string | number, object: object | boolean) {
    if (value) {}
    if (object) {}
}
function nullableBigint(value: bigint | null) {
    if (value) {}
}
function branded(value: (boolean & { readonly brand: unique symbol }) | undefined) {
    if (value) {}
}
