<script setup>
/* should generate diagnostics */
const { foo, bar, baz = 1 } = defineProps({
  foo: Boolean,
  bar: { type: String },
  baz: Number,
  qux: Number,
});
</script>
