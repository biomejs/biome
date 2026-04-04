/* should generate diagnostics */

<script>
import { ref } from 'vue'
const foo = ref(0)
var a = -foo
var b = +foo
var c = !foo
var d = ~foo
</script>
