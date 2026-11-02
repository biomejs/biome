/* should generate diagnostics */
import { ref, ref as createRef } from 'vue'
import * as Vue from 'vue'

const count = ref(0)
const alias = count
const value1 = alias.value

let later
later = count
const value2 = later.value

let assigned
assigned = ref(0)
const value3 = assigned.value

let target
;({ value: target } = count)

const renamed = createRef(0)
const value4 = renamed.value

const namespaced = Vue.ref(0)
const value5 = namespaced.value
