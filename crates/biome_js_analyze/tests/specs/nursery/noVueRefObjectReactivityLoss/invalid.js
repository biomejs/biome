/* should generate diagnostics */
import { ref } from 'vue'
const count = ref(0)
const value1 = count.value
const { value: value2 } = count
const value3 = fn(count.value)
const { value: value4 = 42 } = count
if (foo) {
	const value5 = count.value
}
const value6 = count["value"]
const value7 = count?.value
const value8 = (count).value
count.value.push(1)
