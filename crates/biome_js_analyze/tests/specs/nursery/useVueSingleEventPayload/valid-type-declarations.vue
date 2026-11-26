<!-- should not generate diagnostics -->
<script setup lang="ts">
defineEmits<{
  (e: "change", value: string): void;
  (e: "close"): void;
  update: [value: string];
  open: [];
}>();
</script>
