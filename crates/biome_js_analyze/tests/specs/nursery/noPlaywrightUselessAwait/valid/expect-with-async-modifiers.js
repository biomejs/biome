// Valid: expect with .resolves modifier
await expect(promise).resolves.toBe(1);
await expect(fetchData()).resolves.toEqual({ foo: 'bar' });
await expect(asyncOperation()).resolves.toBeDefined();

// Valid: expect with .rejects modifier
await expect(promise).rejects.toThrow();
await expect(failingAsync()).rejects.toBeInstanceOf(Error);
await expect(badRequest()).rejects.toMatch('error');

// Valid: expect.poll with sync matchers
await expect.poll(() => getValue()).toBe(true);
await expect.poll(() => counter).toBeGreaterThan(5);

// Valid: chained async modifiers
await expect(promise).resolves.not.toBe(null);
await expect(promise).rejects.not.toBeUndefined();

