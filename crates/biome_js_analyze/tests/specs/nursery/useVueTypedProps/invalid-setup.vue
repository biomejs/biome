<!-- should generate diagnostics -->
<script setup>
defineProps({ foo: {}, bar: { required: true } });
const model = defineModel();
const named = defineModel("named");
const empty = defineModel({});
const namedEmpty = defineModel("namedEmpty", { default: "" });
</script>
