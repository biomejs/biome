/* should generate diagnostics - sync expect with .not should not need await */
const x = await expect(value).not.toBe(1);
await expect(str).not.toContain("test");
await expect.soft(value).not.toEqual(expected);
