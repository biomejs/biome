test('example', async ({ page }) => {
    await describe('suite', () => {
        // test content
    });
});

test('example', async ({ page }) => {
    await describe.only('suite', () => {
        // test content
    });
});

test('example', async ({ page }) => {
    await test.describe('suite', () => {
        // test content
    });
});

test('example', async ({ page }) => {
    await test.describe.parallel('suite', () => {
        // test content
    });
});

