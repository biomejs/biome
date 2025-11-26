// should not generate diagnostics
import { computed } from '@ember/object';

export default Component.extend({
  // Has dependencies
  fullName: computed('firstName', 'lastName', function() {
    return `${this.firstName} ${this.lastName}`;
  })
});
