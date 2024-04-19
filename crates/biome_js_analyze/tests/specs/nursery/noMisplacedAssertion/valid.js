describe("msg", () => {
	it("msg", () => {
		expect("something").toBeTrue()
	})
})

test("something", () => {
	expect("something").toBeTrue()
})

Deno.test("something", () => {
	expect("something").toBeTrue()
})

await waitFor(() => {
	expect(111).toBe(222);
});
