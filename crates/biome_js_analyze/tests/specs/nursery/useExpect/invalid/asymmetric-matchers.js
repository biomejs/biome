/* should generate diagnostics */

test("only asymmetric matcher, not an assertion", () => {
    const matcher = expect.stringContaining("foo");
});

test("expect.extend is not an assertion", () => {
    expect.extend({
        toBeWithinRange(received, floor, ceiling) {
            return { pass: received >= floor && received <= ceiling };
        },
    });
});

it("negated asymmetric matcher is not an assertion", () => {
    const matcher = expect.not.objectContaining({ secret: expect.any(String) });
});
