test.describe.parallel.only('focus two tests in parallel mode', () => {
    test('one', async ({ page }) => {});
    test('two', async ({ page }) => {});
});

