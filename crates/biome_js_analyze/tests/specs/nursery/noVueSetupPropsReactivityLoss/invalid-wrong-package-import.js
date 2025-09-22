// toRefs imported from wrong package should be invalid
import { toRefs } from 'some-other-library';

export default {
  setup(props) {
    const { bar } = toRefs(props);
    return () => h('div', bar.value)
  }
}