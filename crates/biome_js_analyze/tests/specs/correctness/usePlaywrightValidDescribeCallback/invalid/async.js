/* should generate diagnostics */
describe('suite', async () => {});
test.describe('suite', async () => {});
test.describe.only('suite', async () => {});
test.describe.skip('suite', async () => {});
test.describe.parallel('suite', async () => {});
test.describe.serial('suite', async () => {});
test.describe.parallel.only('suite', async () => {});
test.describe.serial.only('suite', async () => {});
describe('suite', async function() {});
test.describe('suite', async function() {});
test.describe.fixme('suite', async () => {});
