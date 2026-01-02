<script>
/* should not generate diagnostics */
// defineComponent with only setup() is valid
import { defineComponent, ref } from 'vue'

export default defineComponent({
  setup() {
    const count = ref(0)
    return { count }
  }
})
</script>

