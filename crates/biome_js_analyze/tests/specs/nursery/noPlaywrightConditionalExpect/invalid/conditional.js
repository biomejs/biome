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

test("switch case", async ({ page }) => {
    switch (value) {
        case "a":
            await expect(page).toHaveTitle("Title");
            break;
    }
});
