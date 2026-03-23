/* should generate diagnostics */

// Duplicate test titles inside a describe
describe('foo', () => {
  it('should do bar', () => {});
  it('should do bar', () => {});
});

// Duplicate describe titles inside a describe
describe('suite', () => {
  describe('baz', () => {});
  describe('baz', () => {});
});

// Top-level duplicate tests (no describe wrapper)
it('is awesome', () => {});
it('is awesome', () => {});

// Duplicate no-substitution template literal test titles
describe('template titles', () => {
  it(`same title`, () => {});
  it(`same title`, () => {});
});

// Duplicate describe titles using no-substitution templates
describe(`suite alpha`, () => {});
describe(`suite alpha`, () => {});

// One top-level traversal should still catch nested duplicate scopes
test('top-level anchor', () => {});
describe('outer mixed scope', () => {
  describe('nested suite', () => {
    it('nested duplicate test', () => {});
    it('nested duplicate test', () => {});
  });

  describe('nested suite', () => {});
});
