/* should not generate diagnostics */
expect.hasAssertions;
expect.hasAssertions();
expect.assertions(1);
expect().toBe(true);
expect().toBe();
expect(files).toHaveLength(1);
expect(files).not.toHaveLength(1);
expect(files).to.be.a("array");
expect(files.name).toBe("file");
expect(files["name"]).toBe("file");
expect(files[`name`]).toBe("file");
expect(result).toBe(true);
expect(user.getUserName(5)).resolves.toEqual("Paul");
expect(user.getUserName(5)).rejects.toEqual("Paul");
expect(user.getUserName(5)).not.toEqual("Paul");
expect(a);
expect(files.length);
expect(files.length).toBe;
expect(files.length).not;

// `toHaveLength()` fails on a missing value, which optional chaining allows.
expect(users[0]?.permissions?.length).toBe(1);
expect(files?.length).toBe(1);
expect(files?.list.length).toBe(1);
expect(files?.["length"]).toBe(1);

// Promise modifiers.
expect(files.length).resolves.toBe(1);
expect(files.length).rejects.not.toBe(1);

// Other matchers, argument counts, and unrelated calls.
expect(files.length).toBeGreaterThan(1);
expect(files.length).toBeCloseTo(1);
expect(files.length).toBe();
expect(files.length).toBe(1, 2);
expect(files.length).toBe(...counts);
expect(files.length).not.not.toBe(1);
expect(files.length()).toBe(1);
expect(files.length.value).toBe(1);
expect(files.length + 1).toBe(2);
expect(files[length]).toBe(1);
expect(files.#length).toBe(1);
expect(...files.length).toBe(1);
expect((log(), files.length)).toBe(1);
expect(length).toBe(1);
assert(files.length).toBe(1);
expect.poll(files.length).toBe(1);
expect.soft.only(files.length).toBe(1);
test.soft(files.length).toBe(1);

class Child extends Parent {
	check() {
		// `super` can't be passed to `expect()` without a property access.
		expect(super.length).toBe(1);
	}
}
