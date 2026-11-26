<!-- should generate diagnostics -->
<script setup lang="ts">
const emit = defineEmits<(e: "change" | "save", value: string) => void>();
emit(`change`, "a");
</script>
