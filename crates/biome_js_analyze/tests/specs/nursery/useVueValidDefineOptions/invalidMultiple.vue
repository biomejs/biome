<!-- should generate diagnostics -->
<script setup>
defineOptions({ name: "Foo" });
defineOptions({ inheritAttrs: false });
const options = defineOptions({ name: "Bar" });
</script>
