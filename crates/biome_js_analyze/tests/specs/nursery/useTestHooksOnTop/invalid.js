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

// Hook after test at top level
it('first test', () => {});
beforeEach(() => {});

// Hook after test, with a valid hook earlier (only later one flagged)
describe('qux', () => {
  beforeAll(() => {});
  it('first', () => {});
  afterEach(() => {});
});

// From the ESLint docs: nested describe with hooks out of order
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
