/* should generate diagnostics */
page.locator("button").first();
frame.locator("button").first();
foo.locator("button").first();
foo.first();

page.locator("button").last();
frame.locator("button").last();
foo.locator("button").last();
foo.last();

page.locator("button").nth(3);
frame.locator("button").nth(3);
foo.locator("button").nth(3);
foo.nth(32);

page.locator("button")["first"]();
page.locator("button")['last']();
page.locator("button")[`nth`](1);
page.locator("button")[("first")]();

page.locator("button").first().last();
page.locator("button").nth(1).locator("span").first();

page.locator("button")?.first();
page.locator("button").last?.();
page.locator("button")?.["nth"](2);

(page.locator("button").first)();

await page.locator("button").first().click();
await expect(page.getByRole("listitem").last()).toBeVisible();

buttons.nth(0);
this.buttons.first();

page
	.locator("button")
	// comment
	.nth(
		index,
	);
