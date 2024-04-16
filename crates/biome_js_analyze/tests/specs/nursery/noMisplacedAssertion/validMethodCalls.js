it.each([1, 2, 3])('test', (a, b, expected) => {
  expect(a + b).toBe(expected)
})

test.each([1, 2, 3])('test', (a, b, expected) => {
  expect(a + b).toBe(expected)
})

it.each`
	a               | b      | expected
	${{ val: 1 }}   | ${'b'} | ${'1b'}
`('test', ({ a, b, expected }) => {
  expect(a.val + b).toBe(expected)
})

test.each`
	a               | b      | expected
	${{ val: 1 }}   | ${'b'} | ${'1b'}
`('test', ({ a, b, expected }) => {
  expect(a.val + b).toBe(expected)
})

describe.skip('test', () => {
  test('test', () => {
    assert.equal(Math.sqrt(4), 3)
  })
})

test.skip('test', () => {
  assert.equal(Math.sqrt(4), 2)
})
