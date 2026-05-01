// Standard skip with string first arg — still reported even with allowConditional: true
test.skip("standard skip", () => {});
it.skip("standard skip", () => {});
describe.skip("standard skip", () => {});

// 0-arg bare skip OUTSIDE an `if` block — still reported
test('outside-if', () => {
    test.skip();
});

// describe.skip with non-string first arg — still reported (option only allows test/it)
describe('describe-conditional', () => {
    describe.skip(someCondition);
});

// x-prefix variants are not conditional patterns — still reported
xtest('x-prefix', () => {});
xit('x-prefix', () => {});
xdescribe('x-prefix', () => {});

// Playwright describe/step skip — still reported
test.describe.skip("playwright", () => {});
test.step.skip("playwright", () => {});
