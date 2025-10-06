/* should not generate diagnostics */

// Valid: using props parameter directly
export default {
  setup(props) {
    return () => h('div', props.count)
  }
}
