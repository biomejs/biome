/* should generate diagnostics */

// Empty string titles
describe('', () => {});
it('', () => {});
test('', () => {});

// Empty template literals
it(``, () => {});
describe(``, () => {});

// No arguments at all
it();
describe();

// Accidental whitespace (leading / trailing)
it(' leading space', () => {});
it('trailing space ', () => {});
describe('  both spaces  ', () => {});
it(` template with space `, () => {});

// Accidental space in template literal with substitutions
it(` leading substitution ${x}`, () => {});
it(`${x} trailing substitution `, () => {});

// Non-string titles
it(123, () => {});
it(true, () => {});
it(null, () => {});
describe(123, () => {});
describe(MyComponent, () => {});

// .each calls with invalid titles
test.each([[1, 2]])('', () => {});
test.each([[1, 2]])(' leading space', () => {});
describe.each([[1, 2]])(123, () => {});
