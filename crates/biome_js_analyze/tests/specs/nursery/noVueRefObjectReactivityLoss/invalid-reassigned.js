/* should generate diagnostics */
import { ref } from 'vue'
let count = ref(0)
count = ref(1)
console.log(count.value)

function reset() {
	other = ref(0)
	console.log(other.value)
}
let other = ref(0)
console.log(other.value)
