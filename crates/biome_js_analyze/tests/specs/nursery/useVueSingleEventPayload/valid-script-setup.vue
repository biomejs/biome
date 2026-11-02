<!-- should not generate diagnostics -->
<script setup>
const emit = defineEmits(["change"]);
emit("change");
emit("change", value);
emit("change", { a, b });

const other = { emit() {} };
other.emit("change", a, b);

function send(emit) {
  emit("change", a, b);
}
</script>
