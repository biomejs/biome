<!-- should generate diagnostics -->
<script setup>
const emit = defineEmits([`click`, "change"]);
emit(`click`);
emit("keydown");
(emit)?.("CHANGE");
</script>
