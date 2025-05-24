/* should not generate diagnostics */
it.todo('will test something eventually');

test.todo('will test something eventually');

['x']();

it('should pass', () => expect(true).toBeDefined());

test('should pass', () => expect(true).toBeDefined());

it('should pass', () => somePromise().then(() =>
	expect(true).toBeDefined()
));

afterEach(() => {});

test('should pass', () => {
  expect(true).toBeDefined();
  foo(true).toBe(true);
});

it('should return undefined', () => expectSaga(mySaga).returns());

test('verifies expect method call', () => expect$(123));

test('verifies expect method call', () => new Foo().expect(123));

test('verifies deep expect method call', () => {
  tester.foo().expect(123);
});

test('verifies chained expect method call', () => {
  tester
    .foo()
    .bar()
    .expect(456);
});

test('verifies the function call', () => {
	td.verify(someFunctionCall())
});

it('should pass', () => expect(true).toBeDefined());
