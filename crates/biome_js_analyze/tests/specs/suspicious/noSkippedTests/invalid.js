describe.skip("test", () => {});
it.skip("test", () => {});
test.skip("test", () => {});
xdescribe('foo', () => {});
xit('foo', () => {});
xtest('foo', () => {});

// Playwright: fixme annotations
test.fixme("needs fixing", async () => {});
it.fixme("it needs fixing", async () => {});
describe.fixme("fixme suite", () => {});

// Playwright: test.describe patterns
test.describe.skip("skipped suite", () => {});
test.describe.fixme("fixme suite", () => {});
test.describe.parallel.skip("parallel skip", () => {});
test.describe.serial.skip("serial skip", () => {});
test.describe.parallel.fixme("parallel fixme", () => {});
test.describe.serial.fixme("serial fixme", () => {});

// Playwright: step patterns
test("test with skipped step", async () => {
    await test.step.skip("skipped step", async () => {});
    await test.step.fixme("fixme step", async () => {});
});

// Bracket notation
test["skip"]("bracket notation", async () => {});

// Bare test.skip() with no arguments
test("bare skip", async () => {
    test.skip();
});
