/* should generate diagnostics */
// Test nested element handle calls
const button1 = await (await page.$("button"));

const button2 = await page.$("#foo");

await (await page.$$("div"));

let handle;
handle = await page.$("input");
