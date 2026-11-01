/* should generate diagnostics */
test.describe('suite', { tag: '@smoke' }, async () => {});
test.describe('suite', { tag: '@smoke' }, (done) => {});
test.describe('suite', { tag: '@smoke' }, 'not a function');
test.describe('suite', () => {}, { tag: '@smoke' });
