/* should not generate diagnostics */

<script>
import { ref } from 'vue'
export default {
	setup() {
		const count = ref(0)
		console.log(count.value) // 0

		count.value++
		console.log(count.value) // 1
		return {
			count
		}
	}
}
</script>
