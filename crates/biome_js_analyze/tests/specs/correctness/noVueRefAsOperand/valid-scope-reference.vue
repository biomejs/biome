/* should not generate diagnostics */

<script>
import { ref } from 'vue'
const count = ref(0)
function foo() {
  let count = 0
  count++
}
</script>
