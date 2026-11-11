/* should generate diagnostics */

// Hook after test in describe
describe('foo', () => {
  it('does something', () => {});
  beforeEach(() => {});
});

// Multiple hooks after test
describe('bar', () => {
  it('test one', () => {});
  beforeAll(() => {});
  afterEach(() => {});
});

// `node:test` hooks
describe('qux', () => {
  it('first', () => {});
  before(() => {});
  after(() => {});
});

// Hook after test at top level
it('first test', () => {});
beforeEach(() => {});

// Hook after test, with a valid hook earlier (only later one flagged)
describe('qux', () => {
  beforeAll(() => {});
  it('first', () => {});
  afterEach(() => {});
});

// nested describe with hooks out of order
describe('outer', () => {
  beforeEach(() => {});
  it('accepts this input', () => {});
  beforeAll(() => {});
  describe('inner', () => {
    beforeEach(() => {});
    it('accepts that input', () => {});
    afterEach(() => {});
    beforeEach(() => {});
  });
});

// One top-level traversal should still catch nested violations
describe('mixed top level owner', () => {
  describe('nested suite', () => {
    test('nested test', () => {});
    afterAll(() => {});
  });
});

// Hook after test in suite
suite('my suite', () => {
  it('does something', () => {});
  beforeEach(() => {});
});

// Hook after test in a describe block with a mode
test.describe.serial('serial suite', () => {
  test('does something', () => {});
  test.beforeEach(() => {});
});

// Hook after test in a conditional describe block
describe.skipIf(isCI)('conditional suite', () => {
  it('does something', () => {});
  beforeEach(() => {});
});
