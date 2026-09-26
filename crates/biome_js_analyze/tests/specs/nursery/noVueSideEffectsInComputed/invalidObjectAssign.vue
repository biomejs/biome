<script setup>
import { computed } from 'vue'

const state = useState()

const inline = computed(() => Object.assign(state.data, { extra: 'value' }))
const returned = computed(() => {
  return Object.assign(state.user, state.updates)
})
</script>
