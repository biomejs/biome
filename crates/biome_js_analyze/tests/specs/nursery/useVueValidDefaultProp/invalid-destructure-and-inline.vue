<script setup>
/* should generate diagnostics */
const { foo = 123 } = defineProps({
  foo: { type: String, default: 123 },
});
</script>
