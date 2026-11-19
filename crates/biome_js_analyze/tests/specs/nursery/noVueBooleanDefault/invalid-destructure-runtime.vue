<script setup>
/* should generate diagnostics */
const { foo = true } = defineProps({
  foo: Boolean,
});
</script>
