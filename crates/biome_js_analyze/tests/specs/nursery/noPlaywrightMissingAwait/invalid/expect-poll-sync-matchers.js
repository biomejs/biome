/* should generate diagnostics */
// expect.poll with synchronous matchers should still require await
test('toBe matcher', async () => {
    expect.poll(() => getValue()).toBe(42);
});

test('toEqual matcher', async () => {
    expect.poll(() => getObject()).toEqual({ foo: 'bar' });
});

test('toMatch matcher', async () => {
    expect.poll(() => getString()).toMatch(/pattern/);
});

test('toStrictEqual matcher', async () => {
    expect.poll(() => getData()).toStrictEqual({ a: 1 });
});
