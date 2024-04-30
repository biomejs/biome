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

expect.any(Number);

expect.anything()

expect.closeTo(0.3, 5)

expect.arrayContaining(['Alice', 'Bob'])

expect.objectContaining({
  x: expect.any(Number),
  y: expect.any(Number),
})

expect.stringContaining('Hello world!')

expect.stringMatching(/^Alic/)
expect.stringMatching(/^[BR]ob/)

expect.extend({
  toBeFoo: (received, expected) => {
	if (received !== 'foo') {
      return {
	    message: () => `expected ${received} to be foo`,
		pass: false,
	  }
	}
  },
});

expect.addEqualityTesters([areVolumesEqual]);

expect.addSnapshotSerializer(serializer);