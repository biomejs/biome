<!-- should not generate diagnostics -->
<!-- <script setup> syntax is the recommended Vapor Mode compatible pattern -->
<script setup>
import { ref } from 'vue'

const count = ref(0)

function increment() {
  count.value++
}
</script>

