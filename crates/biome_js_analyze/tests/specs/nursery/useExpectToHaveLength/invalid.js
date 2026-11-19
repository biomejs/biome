/* should generate diagnostics */
expect(files.length).toBe(1);
expect(files.length).toEqual(1);
expect(files.length).toStrictEqual(1);
expect(files.length).not.toBe(1);
expect(files.length).not.toStrictEqual(1);
expect(files["length"]).toBe(1);
expect(files["length"]).toBe(1,);
expect(files["length"])["not"].toBe(1);
expect(files["length"])["toBe"](1);
expect(files["length"]).not["toBe"](1);
expect(files["length"])["not"]["toBe"](1);
expect(files.length)['toEqual'](1);
expect(files.length)[`toEqual`](1);
expect(files[`length`]).toBe(1);
expect(files.length).toBe(expected.length);
expect(files.length, "custom message").toBe(1);
expect(foo.bar.length).toBe(3);
expect(getFiles().length).toBe(3);
expect("text".length).toBe(4);
expect([1, 2].length).toBe(2);
expect(this.length).toBe(1);

// Playwright and Vitest soft assertions.
expect.soft(files.length).toBe(1);
expect.soft(files["length"]).not.toBe(1);

// Parentheses around the argument are removed, parentheses around the value are kept.
expect((files.length)).toBe(1);
expect(((files).length)).toBe(1);
expect((await table.rows.all()).length).toBe(5);
expect((first, second).length).toBe(5);

test("comments around the assertion are kept", () => {
	// leading comment
	expect(files.length).toBe(1); // trailing comment
	expect(/* before */ files /* after */.length).toBe(1);
	expect(files.length)
		// why this is checked
		.toBe(/* expected */ 1);
	expect(files.length) /* modifier */.not /* matcher */
		.toBe(1);
	expect(/* before */ (files.length)).toBe(1);
});

test("no fix when comments would be removed", () => {
	expect(files./* property */ length).toBe(1);
	expect(files.length /* after */).toBe(1);
	expect(files[/* property */ "length"]).toBe(1);
	expect(
		files
			// property
			.length,
	).toBe(1);
	expect((/* inside */ files.length)).toBe(1);
	expect((files.length) /* after */).toBe(1);
});
