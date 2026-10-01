<script setup>
import { computed, reactive } from 'vue'

const state = useState()
const nested = { inner: {} }
const list = reactive([])
function helper() {}
class Helper {}

const assign = computed(() => {
  state.firstName = 'lorem'
  nested.inner.value = 'lorem'
  return state.firstName + ' ' + state.lastName
})
const update = computed(() => {
  state.count += 2
  state.count++
  return state.count
})
const mutatingMethod = computed(() => state.items.reverse())
const directMutatingMethod = computed(() => list.reverse())
const nestedChain = computed(() => {
  const pushed = state.outer.items.push('example')
  return pushed
})
const computedMember = computed(() => {
  state.items[index] = state.other[index]
  return state.items
})
const getterMember = computed({
  get() {
    return state.items.reverse()
  },
})
const arrowGetterMember = computed({
  get: () => state.items.sort(),
})
const functionBinding = computed(() => {
  helper.displayName = 'renamed'
})
const classBinding = computed(() => {
  Helper.displayName = 'renamed'
})
const sequence = computed(() => ((state.flag = true), state.flag))
</script>
