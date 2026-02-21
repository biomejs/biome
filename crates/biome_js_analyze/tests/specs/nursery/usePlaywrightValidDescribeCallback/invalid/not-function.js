/* should generate diagnostics */
describe('suite', 'not a function');
test.describe('suite', 123);
test.describe.only('suite', {});
test.describe.skip('suite', null);
test.describe.parallel('suite', []);
