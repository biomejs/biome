// should not generate diagnostics

// Third-party with 'mixins' in name (not in path)
import mixin from 'lodash-mixins';

export default mixin({}, { a: 1 });
