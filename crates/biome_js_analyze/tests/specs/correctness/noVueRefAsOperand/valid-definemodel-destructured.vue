/* should not generate diagnostics */

<script setup>
const [model, mod] = defineModel();
console.log(model);
function process() {
  if (model.value) console.log('foo')
}
function update(value) {
  model.value = value;
}
</script>
