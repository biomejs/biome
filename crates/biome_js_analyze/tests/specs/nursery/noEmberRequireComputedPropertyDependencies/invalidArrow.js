import { computed } from '@ember/object';

const MyMixin = {
  // Arrow function without dependencies
  displayName: computed(() => {
    return 'No dependencies';
  })
};
