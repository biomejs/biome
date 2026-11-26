/* should not generate diagnostics */
import { ref } from 'vue'
const count = ref(0)
;(async () => {
	const value1 = count.value
})()
;(function* () {
	const value2 = count.value
})()
;(() => {
	const nested = ref(0)
	const value3 = computed(() => nested.value)
	function fn() {
		console.log(nested.value)
	}
})()
