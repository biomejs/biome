/* should generate diagnostics */
// Test $eval and $$eval with multiple arguments
await page.$eval(".main-container", (e, suffix) => e.outerHTML + suffix, "hello");

await page.$$eval("div", (divs, min) => divs.length >= min, 10);

await page.$eval("#search", (el, prop) => el[prop], "value");

await this.page.$$eval("span", (els, className) => els.filter(e => e.className === className), "active");
