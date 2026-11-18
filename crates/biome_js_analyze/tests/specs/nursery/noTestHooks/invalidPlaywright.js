/* should generate diagnostics */
import { test as base } from "@playwright/test";

const test = base.extend({});

test.beforeEach(async ({ page }) => {
	await page.goto("/");
});

test.describe("checkout", () => {
	test.afterAll(async () => {});
});
