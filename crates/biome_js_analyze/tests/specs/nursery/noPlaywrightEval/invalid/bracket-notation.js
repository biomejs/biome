// Test bracket notation for $eval and $$eval
await page["$eval"]("#search", el => el.value);

await page[`$eval`]("#search", el => el.value);

await page["$$eval"]("div", els => els.length);

await page[`$$eval`]("div", els => els.length);

await this.page["$eval"]("#input", el => el.checked);

await this.page[`$$eval`]("span", els => els.map(e => e.textContent));
