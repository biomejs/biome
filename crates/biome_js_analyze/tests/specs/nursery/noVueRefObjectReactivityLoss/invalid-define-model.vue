/* should generate diagnostics */
<script setup>
const model1 = defineModel();
const [model2, modifiers] = defineModel();
console.log(
	model1.value,
	model2.value,
);
let model3;
[model3] = defineModel();
console.log(model3.value);
</script>
