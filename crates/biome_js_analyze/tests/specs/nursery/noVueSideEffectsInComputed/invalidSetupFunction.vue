<script>
import { computed } from 'vue'

export default {
  setup() {
    const state = useState()

    const mutatingMethod = computed(() => state.items.reverse())
    const assign = computed(() => {
      state.firstName = 'lorem'
      return state.firstName
    })
    return { mutatingMethod, assign }
  },
}
</script>
