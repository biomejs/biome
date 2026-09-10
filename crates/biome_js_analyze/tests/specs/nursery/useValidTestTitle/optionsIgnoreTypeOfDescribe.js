/* should generate diagnostics */
// Describe and suite non-string titles are allowed
describe(MyComponent, () => {});
suite(MyComponent, () => {});
describe(6, () => {});

// Test non-string titles are still invalid
it(MyComponent, () => {});
test(123, () => {});
