/* should generate diagnostics */
expect(a === b).toBe(true);
expect(a === b,).toBe(true,);
expect(a === b).toBe(false);
expect(a === b).not.toBe(true);
expect(a === b).not.toBe(false);
expect(a !== b).toBe(true);
expect(a !== b).toBe(false);
expect(a !== b).not.toBe(true);
expect(a !== b).not.toBe(false);
expect(a === b).toEqual(true);
expect(a === b).toStrictEqual(false);
expect(a === b)["toBe"](true);
expect(a === b)["not"].toBe(true);
expect(a === b).resolves.toBe(true);
expect(a === b).resolves.toBe(false);
expect(a === b).rejects.not.toBe(true);
expect(a === b).resolves.not.toBe(false);
expect(a === b)["resolves"].not.toBe(false);
expect(a === b)["resolves"]["not"]["toBe"](false);
expect(a.b() === c.d).toBe(true);
expect(a === b, "custom message").toBe(true);

expect((a === b)).toBe(true);
expect(a === b).toBe((false));

test("comments around the assertion are kept", () => {
	// leading comment
	expect(a === b).toBe(true); // trailing comment
});

test("no fix when comments would be removed", () => {
	expect(a /* left */ === b).toBe(true);
	expect(a === b).not /* negated */.toBe(false);
});
