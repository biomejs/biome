/* should generate diagnostics */
expect(something).toEqual(somethingElse);
expect(something).toEqual(somethingElse,);
expect(something).toEqual();
expect(something, "custom message").toEqual(somethingElse);
expect({ a: "a", b: undefined }).toEqual({ a: "a" });

// Computed matcher names keep their quotes.
expect(something)["toEqual"](somethingElse);
expect(something)['toEqual'](somethingElse);
expect(something)[`toEqual`](somethingElse);

// Modifiers.
expect(a).not.toEqual(b);
expect(a)["not"].toEqual(b);
expect(a).resolves.toEqual(b);
expect(a).rejects.toEqual(b);
expect(a).resolves.not.toEqual(b);
expect(a).rejects.not.toEqual(b);
expect(a)["resolves"]["not"]["toEqual"](b);

// Other ways to call `expect`.
expect.soft(a).toEqual(b);
expect.soft(a).not.toEqual(b);
expect.poll(() => a).toEqual(b);
expect.soft.poll(() => a).toEqual(b);
expect.element(a).toEqual(b);
expect["soft"](a).toEqual(b);

// Parentheses.
(expect(a)).toEqual(b);
(expect(a).toEqual)(b);
(expect(a).not).toEqual(b);
(expect)(a).toEqual(b);
(expect.soft)(a).toEqual(b);

test("nested in a test", async () => {
	expect(a).toEqual(b);
	await expect(a).resolves.toEqual(b);
	expect(a).toEqual(expect(b).toEqual(c));
});

test("comments are kept", () => {
	// leading comment
	expect(a).toEqual(b); // trailing comment
	expect(a) /* before dot */ . /* before name */ toEqual /* after name */ (b);
	expect(a)
		// why this is checked
		.toEqual(b);
	expect(a)[/* before name */ "toEqual" /* after name */](b);
});
