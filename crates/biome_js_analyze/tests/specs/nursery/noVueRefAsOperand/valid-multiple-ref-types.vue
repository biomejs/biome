/* should not generate diagnostics */

<script>
import { ref, computed, toRef, toRefs, reactive, customRef, shallowRef } from 'vue'

let count = ref(0)
let cntcnt = computed(() => count.value + count.value)
count.value++
cntcnt.value++

const state = { foo: 1 }
const fooRef = toRef(state, 'foo')
console.log(fooRef.value)
`${fooRef.value}`

const state2 = reactive({
  foo2: 1,
  bar2: 2
});
const refs = toRefs(state2)
console.log(refs.foo2.value)
const { foo2 } = toRefs(state2)
console.log(`${foo2.value}`)

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
console.log(`${cref.value}`)

const foo = shallowRef({});
const n = foo.value + 1
</script>
