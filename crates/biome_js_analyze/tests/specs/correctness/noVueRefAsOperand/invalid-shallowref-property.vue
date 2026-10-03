/* should generate diagnostics */

<script>
import { shallowRef } from 'vue'
const foo = shallowRef({})
foo.bar = 123
</script>
