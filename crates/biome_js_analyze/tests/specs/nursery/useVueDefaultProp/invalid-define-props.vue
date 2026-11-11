<script setup>
/* should generate diagnostics */
defineProps({
  foo: String,
  bar: {
    type: Number,
    required: false,
  },
});
</script>
