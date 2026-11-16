/* should not generate diagnostics */
function before(message) {
	console.log(message);
}
before("starting");

const after = (callback) => callback();
after(() => {});

const beforeAll = createHook();
beforeAll(() => {});

const { afterAll } = hooks;
afterAll(() => {});

function run(beforeEach, afterEach) {
	beforeEach(() => {});
	afterEach(() => {});
}
