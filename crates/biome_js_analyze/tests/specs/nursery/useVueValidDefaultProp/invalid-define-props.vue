<script setup lang="ts">
/* should generate diagnostics */
defineProps({
  a: { type: String, default: () => 123 },
  b: { type: Array, default: [] },
  c: { type: Number, default: "1" },
});
</script>
