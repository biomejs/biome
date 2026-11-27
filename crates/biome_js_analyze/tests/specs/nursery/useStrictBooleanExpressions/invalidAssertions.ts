// should generate diagnostics
declare function assert(value: unknown): asserts value;
declare function assertSecond(message: string, value: unknown): asserts value;
function checks(number: number | null, string: string | undefined, boolean?: boolean) {
    assert(number);
    assertSecond("present", string);
    assert(boolean);
}
// Each check below runs before any assertion narrows its operands.
function logicalAnd(number: number | null) {
    assert(true && number);
}
function logicalOr(number: number | null, string: string | undefined) {
    assert(number || string);
}
function parenthesized(boolean?: boolean) {
    assert((boolean));
}
