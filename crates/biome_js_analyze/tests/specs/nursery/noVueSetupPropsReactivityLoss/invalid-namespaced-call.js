// namespaced call without proper import should be invalid
export default {
  setup(props) {
    const { qux } = SomeLib.toRefs(props);
    return () => h('div', qux.value)
  }
}