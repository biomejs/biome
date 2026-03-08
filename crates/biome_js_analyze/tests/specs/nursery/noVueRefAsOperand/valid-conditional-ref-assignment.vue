/* should not generate diagnostics */

<script>
import { ref } from 'vue'
let foo;

if (!foo) {
  foo = ref(5);
}
</script>
