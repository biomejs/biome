<!-- <script setup> syntax is the recommended Vapor Mode compatible pattern -->
<script setup>
// should not generate diagnostics
import { ref } from 'vue'

const count = ref(0)

function increment() {
  count.value++
}
</script>

