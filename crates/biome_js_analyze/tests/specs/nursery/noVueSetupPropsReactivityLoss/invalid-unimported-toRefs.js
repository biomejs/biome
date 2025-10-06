// toRefs not imported should be invalid
export default {
  setup(props) {
    const { foo } = toRefs(props);
    return () => h('div', foo.value)
  }
}