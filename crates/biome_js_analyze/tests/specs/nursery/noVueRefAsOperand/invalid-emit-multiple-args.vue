/* should generate diagnostics */

<script>
import { ref, defineComponent } from 'vue'

export default defineComponent({
  emits: ['incremented'],
  setup(_, { emit }) {
    const counter = ref(0)

    emit('incremented', 'xxx', counter)

    return {
      counter
    }
  }
})
</script>
