/* should generate diagnostics */
// Test selectOption with force option (it has different signature with first options argument)
await page.locator("select").selectOption({ label: "Blue" }, { force: true });

await page.locator("select").selectOption("value", { force: true });

await page.locator("select").selectOption(["option1", "option2"], { force: true });
