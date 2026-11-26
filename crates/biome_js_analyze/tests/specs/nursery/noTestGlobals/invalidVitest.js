/* should generate diagnostics */
suite("sum", () => {
	aroundAll(async (runSuite) => {
		await runSuite();
	});
	aroundEach(async (runTest) => {
		await runTest();
	});
	beforeEach(() => {
		vi.useFakeTimers();
	});

	test("adds two numbers", () => {
		onTestFinished(() => {});
		onTestFailed(() => {});
		expect(1 + 2).toBe(3);
		assert.equal(1 + 2, 3);
		expectTypeOf(1 + 2).toBeNumber();
		assertType(1 + 2);
	});
});

vitest.setConfig({ testTimeout: 1000 });
chai.config.truncateThreshold = 0;
