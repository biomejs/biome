<script setup>
/* should not generate diagnostics */
defineProps({
  foo: {
    type: String,
    default: "",
  },
  bar: {
    type: Number,
    required: true,
  },
  baz: Boolean,
});
defineModel();
defineModel("count", { type: Number });
</script>
