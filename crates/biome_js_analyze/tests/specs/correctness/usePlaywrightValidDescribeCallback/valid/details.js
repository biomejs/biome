/* should not generate diagnostics */
test.describe('suite', { tag: '@smoke' }, () => {});
test.describe.only('suite', { tag: ['@smoke', '@slow'] }, () => {});
test.describe.parallel('suite', { annotation: { type: 'issue' } }, function() {});
describe('suite', { tag: '@smoke' }, () => {});
