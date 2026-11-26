<!-- should generate diagnostics -->
<script setup lang="ts">
defineEmits<{
  (e: "change", value1: string, value2: number): void;
  update: [value1: string, value2: number];
}>();
</script>
