<script setup>
/* should not generate diagnostics */
const { foo = 42, bar = "abc" } = defineProps(["foo", "bar"]);
defineEmits(["change"]);
</script>
