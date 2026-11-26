/* should generate diagnostics */
expect(fn).toBeCalledOnce();
expect(fn).toHaveBeenCalledOnce();

// Modifiers
expect(fn).not.toBeCalledOnce();
expect(fn).not.toHaveBeenCalledOnce();
expect(fn).resolves.toBeCalledOnce();
expect(fn).resolves.toHaveBeenCalledOnce();
await expect(fn).rejects.not.toHaveBeenCalledOnce();

// Variants of `expect`
expect.soft(fn).toHaveBeenCalledOnce();
expect.poll(() => fn).toHaveBeenCalledOnce();

// Computed member access
expect(fn)["toBeCalledOnce"]();
expect(fn)[`toHaveBeenCalledOnce`]();
expect(fn)["not"].toHaveBeenCalledOnce();

// Parentheses and comments
(expect(fn)).toHaveBeenCalledOnce();
(expect(fn).toHaveBeenCalledOnce)();
expect(fn) /* before */ .toHaveBeenCalledOnce /* after */ ();
expect(fn).toHaveBeenCalledOnce(/* inside */);
expect(fn)
	.toBeCalledOnce();

// Arguments are ignored by these matchers, so there is no fix
expect(fn).toHaveBeenCalledOnce(sideEffect());

// Inside tests
test("calls the function", () => {
	const mock = vi.fn();
	mock("foo");
	expect(mock).toBeCalledOnce();
	expect(mock).toHaveBeenCalledOnce();
});
