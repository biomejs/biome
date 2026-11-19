/* should generate diagnostics */
describe('suite', (done) => {});
test.describe('suite', (done) => {});
test.describe.only('suite', (done) => {});
test.describe.skip('suite', (done) => {});
test.describe.parallel('suite', (done) => {});
test.describe.serial('suite', (done) => {});
test.describe.parallel.only('suite', (done) => {});
describe('suite', function(done) {});
test.describe('suite', function(done) {});
