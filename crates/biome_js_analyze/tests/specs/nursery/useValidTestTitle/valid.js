/* should not generate diagnostics */

// Valid string titles
describe('my suite', () => {
  it('should work', () => {});
  test('should also work', () => {});
});

// Modifiers and aliases
describe.only('focused suite', () => {});
describe.skip('skipped suite', () => {});
it.only('focused test', () => {});
it.skip('skipped test', () => {});
test.concurrent('concurrent test', () => {});
fit('focused alias', () => {});
xit('skipped alias', () => {});
ftest('focused test alias', () => {});
xtest('skipped test alias', () => {});

// Member calls like test.describe
test.describe('nested suite', () => {});

// Template literal titles without leading/trailing space
it(`template title`, () => {});
describe(`describe template`, () => {});

// Template literal with substitutions
it(`testing user ${userId}`, () => {});
describe(`suite for ${moduleName}`, () => {});

// .each calls
test.each([[1, 2]])('adds %i and %i', () => {});
describe.each([[1, 2]])('suite %i', () => {});

// Spread argument (dynamic, cannot check statically)
it(...args);
