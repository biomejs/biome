const text = await page.locator('.foo').evaluate(el => el.textContent);
const texts = await page.locator('.foo').evaluateAll(els => els.map(el => el.textContent));

