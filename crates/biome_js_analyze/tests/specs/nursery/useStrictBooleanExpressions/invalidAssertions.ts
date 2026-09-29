// should generate diagnostics
declare function assert(value: unknown): asserts value;
declare function assertSecond(message: string, value: unknown): asserts value;
function checks(number: number | null, string: string | undefined, boolean?: boolean) {
    assert(number);
    assertSecond("present", string);
    assert(boolean);
    assert(true && number);
    assert(number || string);
    assert((boolean));
}
