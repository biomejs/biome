/* should not generate diagnostics */
const component = {
  render() {
    return this.$slots.default(foo, bar);
  },
};

function render() {
  return this.$slots.default(...foo);
}
