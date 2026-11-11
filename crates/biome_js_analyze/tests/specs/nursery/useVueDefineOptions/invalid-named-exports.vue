<!-- should generate diagnostics -->
<script>
export const A = 42;
export default { name: "Foo" };
</script>

<script setup>
const props = defineProps(["foo"]);
</script>
