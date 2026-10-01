<script setup>
import { computed } from 'vue'

const state = useState()

const parenthesizedOptionalChain = computed(() => (state?.items)?.sort?.())
const removed = computed(() => {
  delete state.map.key
  return state.map
})
const deepChain = computed(() => {
  state.a.b.c = 'lorem'
  return state.a
})
</script>
