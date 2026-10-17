<!-- should generate diagnostics -->
<script setup lang="ts">
log()
const props = defineProps({});
[1].forEach(log)
defineEmits([]) // Emits.
</script>
