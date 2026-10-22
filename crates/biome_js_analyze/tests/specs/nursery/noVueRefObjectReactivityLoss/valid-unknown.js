/* should not generate diagnostics */
import { ref } from 'vue'
import { ref as otherRef } from 'not-vue'
const [a] = ref(0)
foo.bar = ref(0)
unknown.value
const fromOtherLibrary = otherRef(0)
const value = fromOtherLibrary.value

function ref2() {}
const local = ref2(0)
console.log(local.value)
