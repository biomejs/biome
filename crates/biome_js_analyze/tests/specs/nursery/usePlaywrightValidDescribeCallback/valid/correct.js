/* should not generate diagnostics */

// Basic patterns
describe('suite', () => {});
test.describe('suite', () => {});

// Playwright modifiers
test.describe.only('suite', () => {});
test.describe.skip('suite', () => {});
test.describe.fixme('suite', () => {});

// Playwright execution modes
test.describe.parallel('suite', () => {});
test.describe.serial('suite', () => {});

// Combined mode + modifier
test.describe.parallel.only('suite', () => {});
test.describe.serial.only('suite', () => {});

// Function expressions
describe('suite', function() {});
test.describe('suite', function() {});
test.describe.only('suite', function() {});

// Not describe calls - should be ignored
test.step('step', async () => {});
test.beforeEach(async ({ page }) => {});
test.afterEach(async ({ page }) => {});
test('test', async ({ page }) => {});
someOther.describe('suite', async () => {});
describe.configure({ mode: 'parallel' });
