/* should generate diagnostics */
import { ref } from 'vue'
const count = ref(0)
;(() => {
	const value1 = count.value
})()
;(function () {
	const value2 = count.value
})()
;(() => {
	;(() => {
		const value3 = count.value
	})()
})()
