<script setup lang="ts">
/* should generate diagnostics */
defineEmits<{
  (e: "click"): void;
  (e: "save" | "change", value: string): void;
}>();
</script>
