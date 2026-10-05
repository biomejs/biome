<!-- should generate diagnostics -->
<script setup lang="ts">
const emit = defineEmits<(e: "click" | "save") => void>();
emit("keydown");
</script>
