/* should not generate diagnostics */
page.locator('.my-element');
page.getByRole('button');
expect(1).toBe(1);

