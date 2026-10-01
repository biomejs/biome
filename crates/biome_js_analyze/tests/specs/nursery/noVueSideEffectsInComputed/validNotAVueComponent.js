/* should not generate diagnostics */

import { computed } from 'vue'

const state = { items: [] };

// Not a Vue component, so there is no computed property to check.
export const reordered = computed(() => state.items.reverse());

export const plain = {
  computed: {
    reordered() {
      return this.items.reverse();
    },
  },
};
