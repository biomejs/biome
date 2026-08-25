/* should not generate diagnostics */
test.prop([fc.string()])("round-trips", (s) => {
	expect(s).toBe(s);
});

test.concurrent.prop([fc.string()])("round-trips", (s) => {
	expect(s).toBe(s);
});

test.skip.prop([fc.string()])("round-trips", (s) => {
	expect(s).toBe(s);
});
