/* should not generate diagnostics */

<script setup>
import { computed } from 'vue'
import { helpers } from './helpers'

const state = useState()

const fullName = computed(() => state.firstName + ' ' + state.lastName)
const copiedBeforeReorder = computed(() => state.items.slice(0).reverse())
const localCollection = computed(() => {
  const categories = {}
  state.types.forEach((type) => {
    categories[type.category] = categories[type.category] || []
    categories[type.category].push(type)
  })
  return categories
})
const declaredInGetter = computed(() => {
  const local = []
  local.push('example')
  function inner() {}
  inner.displayName = 'renamed'
  return local
})
const readInNestedCallback = computed(() => state.types.map((type) => type))
const mutationInNestedFunction = computed(() => () => state.items.reverse())
const importedBinding = computed(() => helpers.sort())
const globalBinding = computed(() => globalRegistry.push('example'))
const objectAssignIntoNewObject = computed(() =>
  Object.assign({}, state.data, { extra: 'value' }),
)
const argumentOnly = computed(() => helpers.reorder(state.items))
const byReference = computed(getFullName)

// A call hands the value on, so what comes back is no longer tracked state.
const callBreaksTheChain = computed(() => state.get().items.reverse())
// `state.key` is read as a member name here, it is not written to.
const memberNameOnly = computed(() => registry[state.key])
// `Object.assign` only writes into its first argument.
const objectAssignReadsRest = computed(() => Object.assign({}, state.data))
// The copying array methods return a new array.
const copyingArrayMethod = computed(() => state.items.toReversed())
// Known limitation, shared with the ESLint rule: an alias created by
// destructuring is not followed back to the state it came from.
const destructuredAlias = computed(() => {
  const { items } = state
  items.reverse()
  return items
})
</script>
