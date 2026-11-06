<!-- should not generate diagnostics -->
<script setup>
const definition = { msg: String };
defineProps({ msg: String });

function setup() {
  defineProps(definition);
}
</script>
