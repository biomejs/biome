/* should not generate diagnostics */
// ignored tests don’t count
it.todo('will test something eventually');

test.todo('will test something eventually');

// arbitrary functions are ignored
['x']();

// standard test case
it('should pass', () => {
  expect(true).toBeDefined();
});

// returning directly is valid
test('should pass', () => expect(true).toBeDefined());

// conditional test 1
it('should pass', () => {
  if (foo) {
    expect(true).toBeDefined();
  }
});

// conditional test 2
it('should pass', () => {
  if (foo) {
    return
  } else {
    expect(true).toBeDefined();
  }
});

// conditional test 3
it('should pass', () => {
  if (foo) {
    return
  } else if (bar) {
     expect(true).toBeDefined();
  } else {
    return
  }
});

// conditional loops
it('should pass', () => {
  if (foo) {
    while (foo !== 400) {
      for (let i = 0; i < 10; i++) {
        expect(true).toBeDefined();
      }
    }
  }
});

// named function
it('should pass', function foo() {
  expect(true).toBeDefined();
});

// other test functions don’t need assertions
afterEach(() => {});

beforeAll(() => {});
