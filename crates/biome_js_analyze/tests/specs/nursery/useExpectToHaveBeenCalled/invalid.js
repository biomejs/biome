/* should generate diagnostics */
expect(method).toBeCalledTimes(0);
expect(method).not.toBeCalledTimes(0);
expect(method).toHaveBeenCalledTimes(0);
expect(method).not.toHaveBeenCalledTimes(0);
expect(method).resolves.toHaveBeenCalledTimes(0);
expect(method).rejects.toHaveBeenCalledTimes(0);
expect(method).resolves.not.toHaveBeenCalledTimes(0);
expect(method).rejects.not.toHaveBeenCalledTimes(0);

// Other ways to write the matcher, the modifier, and zero.
expect(method)["toHaveBeenCalledTimes"](0);
expect(method)["not"]["toBeCalledTimes"](0);
expect(method)[`not`].toBeCalledTimes(0);
expect(method).toHaveBeenCalledTimes((0));
expect(method).toHaveBeenCalledTimes(0,);
expect(method).toHaveBeenCalledTimes(0.0);
expect(method).toHaveBeenCalledTimes(0x0);
expect(object.method).toHaveBeenCalledTimes(0);
expect(object?.method).not.toHaveBeenCalledTimes(0);
expect(method, "custom message").toHaveBeenCalledTimes(0);

test("assertions across several lines", () => {
	expect(method)
		.toHaveBeenCalledTimes(0);
	expect(method)
		.not
		.toHaveBeenCalledTimes(0);
});

test("comments around the assertion are kept", () => {
	// leading comment
	expect(method).toHaveBeenCalledTimes(0); // trailing comment
	/* before */ expect(method).not.toHaveBeenCalledTimes(0) /* after */;
});

test("no fix when comments would be removed", () => {
	expect(method).toHaveBeenCalledTimes(/* never */ 0);
	expect(method).not /* at least once */.toHaveBeenCalledTimes(0);
	expect(method)
		// why this is checked
		.toHaveBeenCalledTimes(0);
});

test("no fix when extra arguments would be removed", () => {
	expect(method).not.toHaveBeenCalledTimes(0, 1, 2);
	expect(method).toHaveBeenCalledTimes(0, sideEffect());
});
