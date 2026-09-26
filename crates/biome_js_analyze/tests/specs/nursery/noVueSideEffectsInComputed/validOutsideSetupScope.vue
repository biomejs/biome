/* should not generate diagnostics */

<script>
import { computed } from 'vue'

// `shared` is module state, not setup state, so the Options API component does
// not own it and this rule leaves it alone.
const shared = { items: [] }

export default {
  setup() {
    return {
      reordered: computed(() => shared.items.reverse()),
    }
  },
}
</script>
