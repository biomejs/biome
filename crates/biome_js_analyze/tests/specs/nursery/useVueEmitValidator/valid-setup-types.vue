<script setup lang="ts">
/* should not generate diagnostics */
const emit = defineEmits<{
  (e: "foo", id: number): void;
  bar: [value: string];
}>();
</script>
