/* should generate diagnostics */

<script>
import { ref, computed, toRef, toRefs, reactive, customRef, shallowRef } from 'vue'

let count = ref(0)
let cntcnt = computed(() => count.value + count.value)
count++
cntcnt++

const state = { foo: 1 }
const fooRef = toRef(state, 'foo')
console.log(fooRef)
`${fooRef}`

const state2 = reactive({
  foo2: 1,
  bar2: 2
});
const refs = toRefs(state2)
console.log(refs.foo2)
const { foo2 } = toRefs(state2)
console.log(`${foo2}`)

const cref = customRef((track, trigger) => {
  return {
    get() {
      track()
      return 0
    },
    set(value) {
      trigger()
    }
  }
});
console.log(`${cref}`)

const foo = shallowRef({});
const n = foo + 1
</script>
