<script setup>
/* should generate diagnostics */
const emit = defineEmits([`click`, "change"]);
emit(`click`);
emit("keydown");
(emit)?.("CHANGE");
</script>
