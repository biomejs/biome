/* should generate diagnostics */
// Invalid: sync expect without async modifiers should trigger the rule
await expect(1).toBe(1);
await expect(value).toEqual(expectedValue);
await expect(str).toMatch(/pattern/);
await expect(arr).toHaveLength(3);

// Invalid: expect.soft with sync matcher (soft doesn't make it async)
await expect.soft(value).toBe(123);

