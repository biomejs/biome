import { computed } from '@ember/object';

export default Component.extend({
  // No dependencies - will never update!
  fullName: computed(function() {
    return `${this.firstName} ${this.lastName}`;
  })
});
