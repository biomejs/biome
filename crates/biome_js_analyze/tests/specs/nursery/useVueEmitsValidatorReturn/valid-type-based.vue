<script setup lang="ts">
/* should not generate diagnostics */
const emit = defineEmits<{
  change: [id: number];
  (e: "update", value: string): void;
}>();
</script>
