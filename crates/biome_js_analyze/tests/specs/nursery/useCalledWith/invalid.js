/* should generate diagnostics */
expect(fn).toBeCalled();
expect(fn).toHaveBeenCalled();
expect(fn).toHaveBeenCalledOnce();

// Modifiers other than `not`
expect(fn).resolves.toBeCalled();
expect(fn).rejects.toHaveBeenCalled();
await expect(fn).resolves.toHaveBeenCalledOnce();

// Variants of `expect`
expect.soft(fn).toHaveBeenCalled();
expect.poll(() => fn).toHaveBeenCalled();

// Computed member access
expect(fn)["toBeCalled"]();
expect(fn)[`toHaveBeenCalled`]();
expect(fn)["resolves"].toHaveBeenCalled();

// Parentheses and comments
(expect(fn)).toHaveBeenCalled();
(expect(fn).toHaveBeenCalled)();
expect(fn) /* comment */ .toHaveBeenCalled();
expect(fn)
	.toBeCalled();

// Arguments are ignored by these matchers
expect(fn).toHaveBeenCalled(1, 2);

// Inside tests
it("calls the function", () => {
	expect(mockApi).toHaveBeenCalledOnce();
});
test("calls the method", async () => {
	expect(object.method).toHaveBeenCalled();
	expect(jest.fn()).toBeCalled();
});
