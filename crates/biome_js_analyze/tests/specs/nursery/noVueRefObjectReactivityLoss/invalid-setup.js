/* should generate diagnostics */
import { ref } from 'vue'

export default {
	setup() {
		const count = ref(0)
		const value = count.value
		return { value }
	}
}

function useCounter() {
	const count = ref(0)
	console.log(count.value)
	return count
}
