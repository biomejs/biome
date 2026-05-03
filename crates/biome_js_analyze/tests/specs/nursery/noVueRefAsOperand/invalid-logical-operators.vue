/* should generate diagnostics */

<script>
import { ref } from 'vue'
const foo = ref(true)
const other = true

var a = foo || other
var b = foo && other
</script>
