<!-- should not generate diagnostics -->
<script setup>
import { computed } from 'vue'

const result = computed(() => [{ label: 'a' }].find((item) => item.label === 'a'))
const result2 = computed(() => [{ label: 'a' }].find(({ label }) => label === 'a'))
const result3 = computed(() => [['a']].find(([label]) => label === 'a'))
</script>
