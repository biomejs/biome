/* should not generate diagnostics */
expect.hasAssertions;
expect.hasAssertions();
expect.assertions(1);
expect().toBe(false);
expect(a).toContain(b);
expect(a.name).toBe("b");
expect(a).toBe(true);
expect(a).toEqual(b);
expect(a.test(c)).toEqual(b);
expect(a.includes(b)).toEqual();
expect(a.includes(b)).toEqual("test");
expect(a.includes(b)).toBe("test");
expect(a.includes()).toEqual();
expect(a.includes()).toEqual(true);
expect(a.includes(b, c)).toBe(true);
expect([{ a: 1 }]).toContain({ a: 1 });
expect([1].includes(1)).toEqual;
expect([1].includes).toEqual;
expect([1].includes).not;
expect(a.test(b)).resolves.toEqual(true);
expect(a.test(b)).resolves.not.toEqual(true);
expect(a).not.toContain(b);
expect(a.includes(...[])).toBe(true);
expect(a.includes(b)).toBe(...true);
expect(a);
expect(a).to.be.a("string");

// Optional chaining can't be expressed with toContain().
expect(a?.includes(b)).toBe(true);
expect(a?.b.includes(c)).toBe(true);

// Promise modifiers.
expect(a.includes(b)).resolves.toBe(true);
expect(a.includes(b)).rejects.not.toBe(true);

// Other matchers and unrelated calls.
expect(a.includes(b)).toBeTruthy();
expect(a.includes(b)).toBe(true, false);
expect(a.includes(b)).not.not.toBe(true);
assert(a.includes(b)).toBe(true);
expect(a.includes(b) === true).toBe(true);
expect(a.#includes(b)).toBe(true);
