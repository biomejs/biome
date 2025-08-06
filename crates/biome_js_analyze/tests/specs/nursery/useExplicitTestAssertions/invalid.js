it('should fail', () => {});

it('should fail', myTest);
function myTest() {}

test('should fail', () => {});

test.skip('should fail', () => {});

it('should fail', () => {
	somePromise.then(() => {});
});

test('should fail', () => {
	foo(true).toBe(true);
});

it('should also fail', () => expectSaga(mySaga).returns());
