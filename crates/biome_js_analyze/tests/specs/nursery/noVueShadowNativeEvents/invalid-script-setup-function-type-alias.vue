<!-- should generate diagnostics -->
<script setup lang="ts">
type Emits = (e: "click" | "save", value: string) => void;
defineEmits<Emits>();
</script>
