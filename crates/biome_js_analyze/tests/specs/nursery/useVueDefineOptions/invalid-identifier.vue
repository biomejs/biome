<!-- should generate diagnostics -->
<script>
const options = { name: "Foo" };
export default options;
</script>

<script setup>
const props = defineProps(["foo"]);
</script>
