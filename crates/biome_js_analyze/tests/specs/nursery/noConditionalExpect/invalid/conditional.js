/* should generate diagnostics */

test("if statement", async ({ page }) => {
    if (someCondition) {
        await expect(page).toHaveTitle("Title");
    }
});

test("ternary", async ({ page }) => {
    someCondition ? await expect(page).toHaveTitle("Title") : null;
});

test("logical and", async ({ page }) => {
    someCondition && await expect(page).toHaveTitle("Title");
});

test("logical or", async ({ page }) => {
    someCondition || await expect(page).toHaveTitle("Title");
});

test("nullish coalescing", async ({ page }) => {
    someValue ?? await expect(page).toHaveTitle("Title");
});

test("switch case", async ({ page }) => {
    switch (value) {
        case "a":
            await expect(page).toHaveTitle("Title");
            break;
    }
});

test("if with .not modifier", async ({ page }) => {
    if (someCondition) {
        await expect(page).not.toHaveTitle("Title");
    }
});

test("ternary with .not modifier", async ({ page }) => {
    someCondition ? await expect(page).not.toBeVisible() : null;
});

// .resolves modifier
test("if with .resolves", async ({ page }) => {
    if (someCondition) {
        await expect(fetchData()).resolves.toBeTruthy();
    }
});

// .rejects modifier
test("ternary with .rejects", async ({ page }) => {
    someCondition ? await expect(badRequest()).rejects.toThrow() : null;
});

// .poll() modifier
test("logical and with expect.poll", async ({ page }) => {
    someCondition && await expect.poll(() => getValue()).toBe(true);
});

// .soft() modifier
test("switch with expect.soft", async ({ page }) => {
    switch (value) {
        case "a":
            await expect.soft(page.locator("h1")).toBeVisible();
            break;
    }
});

// Chained modifiers: .resolves.not
test("if with .resolves.not", async ({ page }) => {
    if (someCondition) {
        await expect(fetchData()).resolves.not.toBeNull();
    }
});

// Chained modifiers: .rejects.not
test("ternary with .rejects.not", async ({ page }) => {
    someCondition ? await expect(badRequest()).rejects.not.toBeUndefined() : null;
});

// Chained modifiers: .soft().not
test("switch with expect.soft and .not", async ({ page }) => {
    switch (value) {
        case "a":
            await expect.soft(element).not.toBeHidden();
            break;
    }
});

// Catch clause
test("catch clause", async ({ page }) => {
    try {
        await page.click("button");
    } catch (e) {
        await expect(page).toHaveTitle("Title");
    }
});
