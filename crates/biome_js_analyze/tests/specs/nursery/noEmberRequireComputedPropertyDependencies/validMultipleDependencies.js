// should not generate diagnostics
import { computed } from '@ember/object';

export default class MyClass {
  // Multiple dependencies properly declared
  fullAddress: computed('street', 'city', 'state', 'zip', function() {
    return `${this.street}, ${this.city}, ${this.state} ${this.zip}`;
  })
}
