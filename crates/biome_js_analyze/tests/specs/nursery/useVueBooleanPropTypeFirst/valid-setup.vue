<!-- should not generate diagnostics -->
<script setup lang="ts">
defineProps({
  a: [Boolean, String],
  b: { type: [Boolean, String] },
});
</script>
