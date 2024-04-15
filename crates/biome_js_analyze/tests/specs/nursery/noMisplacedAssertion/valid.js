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

test.each([1, 2, 3])('add(%i, %i) -> %i', (a, b, expected) => {
  expect(a + b).toBe(expected)
})

it.each([1, 2, 3])('add(%i, %i) -> %i', (a, b, expected) => {
  expect(a + b).toBe(expected)
})

test.each`${1} | ${2} | ${3}`('returns $expected when $a is added to $b', ({a, b, expected}) => {
  expect(a + b).toBe(expected);
});

it.each`
  a    | b    | expected
  ${1} | ${1} | ${2}
  ${1} | ${2} | ${3}
  ${2} | ${1} | ${3}
`('returns $expected when $a is added to $b', ({a, b, expected}) => {
  expect(a + b).toBe(expected);
});
