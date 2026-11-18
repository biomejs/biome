/* should not generate diagnostics */
const notAComponent = {
  render() {
    return this.$slots.default.length;
  },
};

function render() {
  return [...this.$slots.default];
}
