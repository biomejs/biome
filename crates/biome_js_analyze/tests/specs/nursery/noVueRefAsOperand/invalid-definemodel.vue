/* should generate diagnostics */

<script>
let model = defineModel();
console.log(model);
function process() {
  if (model) console.log('foo')
}
function update(value) {
  model = value;
}
</script>
