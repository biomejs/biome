const texts = await page.$$eval('.foo', els => els.map(el => el.textContent));

