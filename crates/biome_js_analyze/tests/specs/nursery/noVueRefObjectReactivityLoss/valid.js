/* should not generate diagnostics */
import { ref, toRefs, computed, shallowRef, customRef, toRef } from 'vue'

const count = ref(0)
const value1 = computed(() => count.value)
const value2 = fn(count)
const value3 = computed(() => fn(count.value))
const alias = count
watch(alias, () => console.log(alias.value))

const { other } = toRefs(foo)
const value4 = computed(() => other.value)
const value5 = fn(other)

const refs = toRefs(foo)
const value6 = computed(() => refs.count.value)
const value7 = fn(refs.count)

count.value = 42
count.value++
count.value += 1
count.value.total = 1
count.value.items[0]++
;({ foo: count.value } = bar)
;[count.value] = bar

const r = ref(0)
const c = computed(() => r.value)
const sr = shallowRef({ count: 1 })
const cr = customRef((track, trigger) => {
	return {
		get() { return sr.value },
		set(newValue) { sr.value = newValue }
	}
})
const tr = toRef(sr, 'count')
function fn() {
	console.log(c.value, cr.value, tr.value)
}
class Counter {
	constructor() { this.start = r.value }
	get current() { return r.value }
	read() { return r.value }
}
const handlers = {
	read() { return r.value },
	get current() { return r.value },
}

let looped = ref(0)
looped = looped

const notTracked = { value: 1 }
const value8 = notTracked.value
