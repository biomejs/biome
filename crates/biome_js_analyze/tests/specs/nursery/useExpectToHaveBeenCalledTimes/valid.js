/* should not generate diagnostics */
expect.assertions(1);
expect(method);
expect(method).toHaveBeenCalledTimes;
expect(method).toHaveBeenCalledTimes(0);
expect(method).toHaveBeenCalledTimes(1);
expect(method).not.toHaveBeenCalledTimes(10);
expect(method).resolves.toHaveBeenCalledTimes(10);
expect(method).toBeCalledTimes(0);
expect().toHaveLength(1);
expect(items).toHaveLength(1);

// The matcher isn't called.
expect(method.mock.calls).toHaveLength;
expect(method.mock.calls).not;

// Other matchers.
expect(method.mock.calls).toEqual([]);
expect(method.mock.calls).toContain(1, 2, 3);
expect(method.mock.calls.length).toBe(1);
expect(method.mock.calls).toHaveLength`1`;

// Other values.
expect(method.mock.values).toHaveLength(0);
expect(method.values.calls).toHaveLength(0);
expect(method.mock.length).toEqual(1);
expect(method.mock.calls[0]).toHaveLength(2);
expect(method.mock.calls[0][0]).toStrictEqual(value);
expect(method.mock.calls.slice(1)).toHaveLength(1);
expect(method.mock.calls()).toHaveLength(1);
expect(mock.calls).toHaveLength(1);
expect(calls).toHaveLength(1);
expect(method[mock][calls]).toHaveLength(1);
expect(...method.mock.calls).toHaveLength(1);
expect(other, method.mock.calls).toHaveLength(1);

// Optional chaining can't be expressed with toHaveBeenCalledTimes().
expect(method?.mock.calls).toHaveLength(1);
expect(method.mock?.calls).toHaveLength(1);
expect(my?.method.mock.calls).toHaveLength(1);
expect((method?.mock).calls).toHaveLength(1);

// Unknown modifiers and unrelated calls.
expect(method.mock.calls).to.toHaveLength(1);
expect(method.mock.calls).not().toHaveLength(1);
expect.soft(method.mock.calls).toHaveLength(1);
assert(method.mock.calls).toHaveLength(1);
toHaveLength(method.mock.calls);

// `super` can't be passed to expect() on its own.
class Child extends Parent {
	check() {
		expect(super.mock.calls).toHaveLength(1);
	}
}
