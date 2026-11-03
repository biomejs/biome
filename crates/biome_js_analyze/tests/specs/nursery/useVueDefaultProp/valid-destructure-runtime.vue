<script setup>
/* should not generate diagnostics */
const { foo = 42, bar = 42, baz } = defineProps({
  foo: Number,
  bar: { type: Number, required: false },
  baz: { type: Number, required: true },
});
</script>
