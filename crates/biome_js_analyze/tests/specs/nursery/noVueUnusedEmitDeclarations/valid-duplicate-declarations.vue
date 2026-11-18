<!-- should not generate diagnostics -->
<script setup lang="ts">
const emit = defineEmits<{
  (e: "change", value: string): void;
  (e: "change", value: number): void;
}>();
emit("change", 1);
</script>
