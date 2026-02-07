/* should not generate diagnostics */

<script>
import { ref, defineComponent } from 'vue'

export default defineComponent({
  emits: ['incremented'],
  setup(_, ctx) {
    const counter = ref(0)

    ctx.emit('incremented', counter.value)

    return {
      counter
    }
  }
})
</script>
