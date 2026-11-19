// @fast-check/vitest's `test.prop` is a curried call, shaped just like
// `test.each`/`test.for`: it must use the normal breakable argument layout,
// not the hugged single-line-friendly layout of plain `it`/`test` calls.
test.prop([fc.string()])("round-trips", (s) => {
	expect(s).toBe(s);
});

test.concurrent.prop([fc.string()])(
	"a description that is long enough to push the hugged opening line beyond the print width",
	(s) => {
		expect(s).toBe(s);
	},
);

test.skip.prop([fc.string()])(
	"a description that is long enough to push the hugged opening line beyond the print width",
	(s) => {
		expect(s).toBe(s);
	},
);
