<!-- should not generate diagnostics -->
<script setup>
defineEmits({ "update:foo"() {}, "update:modelValue": null });
const foo = defineModel("foo");
const [model, modifiers] = defineModel();
</script>
