<!-- should not generate diagnostics -->
<script setup lang="ts">
const emit = defineEmits<{
  change(id: number): void;
}>();

emit("change", 1);
</script>
