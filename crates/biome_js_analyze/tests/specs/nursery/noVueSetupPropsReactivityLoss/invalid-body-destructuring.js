// Invalid: destructuring props in setup function body
export default {
  setup(props) {
    const { count } = props;
    return () => h('div', count);
  }
}
