<!-- should generate diagnostics -->
<script setup>
const emit = defineEmits({
  change: (value1, value2) => true,
  update: null,
});
emit("change", value1, value2);
emit("update", a, b, c);
</script>
