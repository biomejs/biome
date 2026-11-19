/* should generate diagnostics */

<script setup>
import { ref } from 'vue'
const emits = defineEmits(['test'])
const count = ref(0)

function update() {
  emits('test', count)
}
</script>
