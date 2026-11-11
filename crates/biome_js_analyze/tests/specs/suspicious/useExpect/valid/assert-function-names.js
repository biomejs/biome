/* should not generate diagnostics */

test("custom assertion function", () => {
    expectSaga(saga).put({ type: "DONE" }).run();
});

test("custom assertion method", () => {
    utils.assertValid(compute());
});

test("nested in a callback", () => {
    items.forEach((item) => {
        utils.assertValid(item);
    });
});

test("built-in assertions still count", () => {
    expect(compute()).toBe(1);
});
