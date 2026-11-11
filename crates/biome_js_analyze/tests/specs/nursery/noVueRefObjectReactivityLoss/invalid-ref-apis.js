/* should generate diagnostics */
import { ref, computed, shallowRef, customRef, toRef, useTemplateRef } from 'vue'
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
const el = useTemplateRef('el')

console.log(
	r.value,
	c.value,
	sr.value,
	cr.value,
	tr.value,
	el.value,
)
