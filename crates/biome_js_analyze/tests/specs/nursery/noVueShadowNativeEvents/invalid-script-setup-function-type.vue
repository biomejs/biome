<script setup lang="ts">
/* should generate diagnostics */
const emit = defineEmits<(e: "click" | "save") => void>();
emit("keydown");
</script>
