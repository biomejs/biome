/* should generate diagnostics */

<script>
import { ref } from 'vue'
let foo = ref(0)
let baz = 0

foo += 1
foo -= 1
baz += foo
baz -= foo
</script>
