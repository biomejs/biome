/* should generate diagnostics */

test("function that is not listed", () => {
    checkResult(compute());
});

test("method name without its object", () => {
    assertValid(compute());
});

test("method on another object", () => {
    helpers.assertValid(compute());
});

test("listed function that is not called", () => {
    const check = expectSaga;
});
