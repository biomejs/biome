/* should generate diagnostics */
expect(a.includes(b)).toEqual(true);
expect(a.includes(b,),).toEqual(true,);
expect(a['includes'](b)).toEqual(true);
expect(a['includes'](b))['toEqual'](true);
expect(a['includes'](b)).toEqual(false);
expect(a['includes'](b)).not.toEqual(false);
expect(a['includes'](b))['not'].toEqual(false);
expect(a['includes'](b))['not']['toEqual'](false);
expect(a.includes(b)).toEqual(false);
expect(a.includes(b)).not.toEqual(false);
expect(a.includes(b)).not.toEqual(true);
expect(a.includes(b)).toBe(true);
expect(a.includes(b)).toBe(false);
expect(a.includes(b)).not.toBe(false);
expect(a.includes(b)).not.toBe(true);
expect(a.includes(b)).toStrictEqual(true);
expect(a.includes(b)).toStrictEqual(false);
expect(a.includes(b)).not.toStrictEqual(false);
expect(a.includes(b)).not.toStrictEqual(true);
expect(a.test(t).includes(b.test(p))).toEqual(true);
expect(a.test(t).includes(b.test(p))).toEqual(false);
expect(a.test(t).includes(b.test(p))).not.toEqual(true);
expect(a.test(t).includes(b.test(p))).not.toEqual(false);
expect([{ a: 1 }].includes({ a: 1 })).toBe(true);
expect([{ a: 1 }].includes({ a: 1 })).not.toBe(true);
expect(a.includes(b), "custom message").toBe(true);

expect((a.includes(b))).toBe(true);
expect(a.includes(b)).toBe((false));

test("comments around the assertion are kept", () => {
	// leading comment
	expect(a.includes(b)).toBe(true); // trailing comment
});

test("no fix when comments would be removed", () => {
	expect(a.includes(/* item */ b)).toBe(true);
	expect(a.includes(b)).not /* negated */.toBe(false);
	expect(a.includes(b))
		// why this is checked
		.toBe(true);
});
