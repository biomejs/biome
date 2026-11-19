/* should not generate diagnostics */

<script setup>
const emit = defineEmits(['test'])
const [model, mod] = defineModel();

function update() {
  emit('test', model.value)
}
</script>
