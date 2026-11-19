/* should not generate diagnostics */

// All hooks before tests
describe('foo', () => {
  beforeAll(() => {});
  beforeEach(() => {});
  afterEach(() => {});
  afterAll(() => {});
  it('does something', () => {});
  it('does another thing', () => {});
});

// No hooks
describe('bar', () => {
  it('first', () => {});
  it('second', () => {});
});

// Empty describe
describe('empty', () => {});

// Top-level hook before test
beforeEach(() => {});
it('top-level test', () => {});

// Nested describe: each scope is valid independently
describe('outer', () => {
  beforeAll(() => {});
  it('outer test', () => {});
  describe('inner', () => {
    beforeEach(() => {});
    it('inner test', () => {});
  });
});

// A top-level describe can own traversal while nested scopes stay valid
describe('top-level owner valid', () => {
  describe('inner valid', () => {
    beforeEach(() => {});
    test('nested test', () => {});
  });
});
