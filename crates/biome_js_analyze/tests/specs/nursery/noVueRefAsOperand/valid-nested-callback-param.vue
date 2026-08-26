<!-- should not generate diagnostics -->
<script setup>
import { computed } from 'vue'

const result = computed(() => [{ label: 'a' }].find((item) => item.label === 'a'))
</script>
