/* should not generate diagnostics */

// Different titles in same describe block
describe('foo', () => {
  it('should do foo', () => {});
  it('should do bar', () => {});
});

// Same title allowed across different describe blocks (not siblings at same level)
describe('outer', () => {
  describe('baz', () => {
    it('should work', () => {});
  });
  describe('bar', () => {
    it('should work', () => {}); // same as sibling describe's test, fine
  });
});

// Parent/child same names are OK (parent is at top level, child is nested)
describe('parent', () => {
  it('should do foo', () => {}); // same name as outer describe, fine
  describe('child', () => {        // different name from parent
    it('should do foo', () => {}); // same title as sibling scope test, fine (different scope)
  });
});

// Top-level tests with different titles
it('is awesome', () => {});
it('is super', () => {});

// Describe with different titles at top level
describe('suite one', () => {});
describe('suite two', () => {});

// test.only and test with different titles in same scope
describe('mixed modifiers', () => {
  test('first test', () => {});
  test.only('second test', () => {});
});

// No-substitution template literals with different titles are fine
describe('template valid', () => {
  it(`first test`, () => {});
  it(`second test`, () => {});
});

// Template literals with substitutions cannot be statically compared — not flagged
const title = 'dynamic';
describe('template dynamic', () => {
  it(`test ${title} one`, () => {});
  it(`test ${title} two`, () => {});
});
