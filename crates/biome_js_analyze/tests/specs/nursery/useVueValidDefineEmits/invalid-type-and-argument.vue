<!-- should generate diagnostics -->
<script setup lang="ts">
const events = ["notify"];

const emit = defineEmits<{ notify: [id: number] }>(events);
</script>
