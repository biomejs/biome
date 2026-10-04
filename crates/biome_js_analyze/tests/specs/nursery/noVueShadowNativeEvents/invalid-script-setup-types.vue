<!-- should generate diagnostics -->
<script setup lang="ts">
defineEmits<{
  (e: "click"): void;
  (e: "save" | "change", value: string): void;
}>();
</script>
