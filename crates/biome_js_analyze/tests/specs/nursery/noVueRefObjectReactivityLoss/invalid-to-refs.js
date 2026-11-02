/* should generate diagnostics */
import { toRefs } from 'vue'
const { count } = toRefs(foo)
const value1 = count.value
const { value: value2 } = count
const value3 = fn(count.value)

const refs = toRefs(foo)
const value4 = refs.count.value
const { value: value5 } = refs.count
const value6 = fn(refs.count.value)

const { foo: renamed = 42, nested: { value: value7 }, ...rest } = refs
const value8 = renamed.value
const value9 = rest.other.value

let assigned
let assignedRest
;({ assigned, ...assignedRest } = toRefs(foo))
const value10 = assigned.value
const value11 = assignedRest.other.value
