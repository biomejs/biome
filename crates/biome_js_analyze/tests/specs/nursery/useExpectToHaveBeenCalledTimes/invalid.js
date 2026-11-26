/* should generate diagnostics */
expect(method.mock.calls).toHaveLength(1);
expect(method.mock.calls).toHaveLength(0);
expect(method.mock.calls).toHaveLength(count);
expect(method.mock.calls).not.toHaveLength(0);
expect(method.mock.calls).resolves.toHaveLength(count);
expect(method.mock.calls).rejects.not.toHaveLength(count);
expect(my.method.mock.calls).not.toHaveLength(0);
expect(jest.mocked(method).mock.calls).toHaveLength(1);
expect(vi.mocked(method).mock.calls).toHaveLength(1);
expect(method.mock.calls, "custom message").toHaveLength(1);
expect(method.mock.calls,).toHaveLength(1,);
expect(method.mock.calls).toHaveLength();
expect(method.mock.calls).toHaveLength(1, 2);
expect(method.mock.calls)?.toHaveLength(1);

// Properties written with brackets.
expect(method["mock"].calls).toHaveLength(0);
expect(method.mock["calls"]).toHaveLength(0);
expect(method[`mock`][`calls`]).toHaveLength(0);
expect(method.mock.calls)["toHaveLength"](1);
expect(method.mock.calls)['toHaveLength'](1);
expect(method.mock.calls)[`toHaveLength`](1);
expect(method.mock.calls)["not"]["toHaveLength"](1);
expect(method.mock.calls)?.[("toHaveLength")](1);

// Parentheses.
expect((method.mock.calls)).toHaveLength(1);
expect((method.mock).calls).toHaveLength(1);
expect((method).mock.calls).toHaveLength(1);
expect((first, second).mock.calls).toHaveLength(1);
(expect(method.mock.calls).toHaveLength)(1);
(expect(method.mock.calls)).toHaveLength(1);
(expect(method.mock.calls).not).toHaveLength(1);

test("comments around the removed properties are kept", async () => {
	// leading comment
	expect(method.mock.calls).toHaveLength(1); // trailing comment
	expect(/* before */ method.mock.calls /* after */).toHaveLength(/* count */ 1);
	expect(method.mock.calls) /* matcher */.toHaveLength /* arguments */(1);
	expect(method.mock.calls)[/* before */ "toHaveLength" /* after */](1);
	await expect(method.mock.calls)
		// why this is checked
		.resolves.toHaveLength(1);
});

test("no fix when comments would be removed", () => {
	expect(method /* mock */.mock.calls).toHaveLength(1);
	expect(method.mock /* calls */.calls).toHaveLength(1);
	expect(
		method
			// the recorded calls
			.mock.calls,
	).toHaveLength(1);
	expect((/* mock */ method.mock.calls)).toHaveLength(1);
});
