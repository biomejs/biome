<script setup>
/* should not generate diagnostics */
const customProps = defineProps(["foo", "bar"]);
const customEmits = defineEmits(["baz"]);
</script>
