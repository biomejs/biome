// Custom extend methods on non-Ember objects should not trigger the rule
// should not generate diagnostics
const myCustomObject = {
  extend(props) {
    return { ...this, ...props };
  }
};

const obj = myCustomObject.extend({
  foo: 'bar'
});
