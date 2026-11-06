<!-- should not generate diagnostics -->
<script setup>
const names = ["foo"];
defineEmits([...names]);
</script>
