test('example', async () => {
    expect.poll(() => foo).toBe(true);
});

