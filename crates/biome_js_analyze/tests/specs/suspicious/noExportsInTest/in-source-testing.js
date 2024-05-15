export const test = "abcdef"

if (import.meta.vitest) {
    const { describe, expect } = import.meta.vitest
    describe("a test", () => {
        expect(test).toEqual("abcdef")
    })
}
