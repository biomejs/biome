/* should generate diagnostics */
await page.click("button", ({ force: true }));
await page.locator('x').fill("text", (({ force: true })));
