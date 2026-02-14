// Invalid: basic parameter destructuring
export default {
  setup({ count }) {
    return () => h('div', count);
  }
}
