/* should generate diagnostics */
// Disallowed words present
it('contains skip keyword', () => {});
describe('only this test', () => {});

// Partial matches should be valid (not whole words)
it('skipped keyword is fine', () => {});
it('lonely keyword is fine', () => {});
