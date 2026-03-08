/* should generate diagnostics */

// beforeEach before beforeAll — wrong order
describe('foo', () => {
    beforeEach(() => {
        seedMyDatabase();
    });

    beforeAll(() => {
        createMyDatabase();
    });

    it('accepts this input', () => {});
    it('returns that value', () => {});
});

// afterAll before afterEach — wrong order
describe('bar', () => {
    afterAll(() => {});
    afterEach(() => {});
});

// Multiple violations in one block
describe('baz', () => {
    afterEach(() => {});
    afterAll(() => {});
    beforeAll(() => {});
});

// Nested describe with out-of-order hooks
describe('outer', () => {
    beforeAll(() => {});

    describe('inner', () => {
        afterAll(() => {});
        afterEach(() => {});
        beforeEach(() => {});
    });

    afterAll(() => {});
});

// Member form: out-of-order
describe('member form', () => {
    test.afterEach(() => {});
    test.beforeEach(() => {});
});

// Top-level out-of-order hooks
afterEach(() => {});
beforeAll(() => {});
