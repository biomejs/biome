test('extracted locator with toBeVisible', async ({ page }) => {
    const loc = page.locator('.item');
    expect(loc).toBeVisible();
});

test('extracted locator with toBeChecked', async ({ page }) => {
    const button = page.getByRole('checkbox');
    expect(button).toBeChecked();
});

test('extracted chained locator', async ({ page }) => {
    const nested = page.locator('.parent').locator('.child');
    expect(nested).toBeVisible();
});

test('extracted locator from frame', async ({ page }) => {
    const el = frame.locator('.item');
    expect(el).toBeVisible();
});

test('not chain with extracted locator', async ({ page }) => {
    const btn = page.getByRole("button");
    expect(btn).not.toBeVisible();
});

test('expect.soft with extracted locator', async ({ page }) => {
    const link = page.getByRole("link");
    expect.soft(link).toBeVisible();
});
