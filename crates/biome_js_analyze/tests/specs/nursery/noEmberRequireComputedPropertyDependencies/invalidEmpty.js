import { computed } from '@ember/object';

export default class MyClass {
  // computed() with function but no dependency keys
  fullName = computed(function() {
    return this.firstName + ' ' + this.lastName;
  });
}
