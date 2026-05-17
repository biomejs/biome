/* should not generate diagnostics */

// Correct order: beforeAll, beforeEach, afterEach, afterAll
describe('foo', () => {
    beforeAll(() => {});
    beforeEach(() => {});
    afterEach(() => {});
    afterAll(() => {});
});

// Partial hooks in correct order
describe('partial hooks', () => {
    beforeEach(() => {});
    afterEach(() => {});
});

// Only afterAll
describe('only afterAll', () => {
    afterAll(() => {});
    it('a test', () => {});
});

// Hooks separated by test cases — reset the comparison window
describe('separated by tests', () => {
    beforeEach(() => {});
    it('a test', () => {});
    afterAll(() => {});
});

// Hooks separated by a describe block — reset the comparison window
describe('separated by describe', () => {
    afterAll(() => {});
    describe('inner', () => {});
    beforeAll(() => {});
});

// Single hook
describe('single hook', () => {
    beforeEach(() => {});
    it('a test', () => {});
});

// Member form: test.beforeAll etc.
describe('member form', () => {
    test.beforeAll(() => {});
    test.beforeEach(() => {});
    test.afterEach(() => {});
    test.afterAll(() => {});
});

// Same hook type repeated (not a violation)
describe('repeated hooks', () => {
    beforeEach(() => { /* first */ });
    beforeEach(() => { /* second */ });
});

// Top-level hooks in correct order
beforeAll(() => {});
beforeEach(() => {});
afterEach(() => {});
afterAll(() => {});

// Nested describe blocks — each checked independently
describe('outer', () => {
    beforeAll(() => {});
    afterAll(() => {});

    describe('inner', () => {
        beforeEach(() => {});
        afterEach(() => {});
    });
});
