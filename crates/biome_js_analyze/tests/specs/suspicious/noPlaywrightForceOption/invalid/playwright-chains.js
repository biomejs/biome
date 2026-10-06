/* should generate diagnostics - Playwright call chains */

// Direct page calls
page.click('button', { force: true });

// Locator chains
page.locator('button').click({ force: true });
page.locator('button').hover({ force: true });

// GetBy methods (locator chains)
page.getByRole('button').click({ force: true });
page.getByTestId('submit').tap({ force: true });
page.getByText('Click me').dblclick({ force: true });

// Frame calls
frame.locator('input').fill('text', { force: true });
frame.getByLabel('Email').click({ force: true });

// Variables ending with Page/Frame
myPage.locator('x').click({ force: true });
childFrame.getByRole('link').hover({ force: true });

// Member access to page
context.page.locator('button').click({ force: true });

// Nested locator chains
page.locator('div').locator('button').click({ force: true });
page.locator('form').first().click({ force: true });
page.locator('li').nth(0).click({ force: true });
