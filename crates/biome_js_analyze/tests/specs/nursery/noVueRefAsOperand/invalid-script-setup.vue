/* should generate diagnostics */

<script setup>
import { ref } from 'vue'
let count = ref(0)

count++ // error
console.log(count + 1) // error
console.log(1 + count) // error
</script>
