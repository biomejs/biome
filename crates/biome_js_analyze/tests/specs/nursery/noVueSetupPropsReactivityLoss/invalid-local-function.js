// locally defined toRefs function should be invalid
function toRefs(obj) {
  return obj;
}

export default {
  setup(props) {
    const { baz } = toRefs(props);
    return () => h('div', baz)
  }
}