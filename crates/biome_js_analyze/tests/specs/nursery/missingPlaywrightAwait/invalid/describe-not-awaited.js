test('example', async ({ page }) => {
    describe('suite', () => {
        // test content
    });
});

test('example', async ({ page }) => {
    describe.only('suite', () => {
        // test content
    });
});

test('example', async ({ page }) => {
    test.describe('suite', () => {
        // test content
    });
});

test('example', async ({ page }) => {
    test.describe.parallel('suite', () => {
        // test content
    });
});

