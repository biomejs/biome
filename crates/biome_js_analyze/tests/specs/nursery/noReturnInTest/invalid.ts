/* should generate diagnostics */
test("type assertion", (() => {
	return expect(1).toBe(1);
}) as TestFunction);

test("non-null assertion", typedTest!);
function typedTest(): Promise<void> {
	return expect(1).resolves.toBe(1);
}
