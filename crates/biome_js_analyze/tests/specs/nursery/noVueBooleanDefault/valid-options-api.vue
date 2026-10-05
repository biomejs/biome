<script>
/* should not generate diagnostics */
export default {
  props: {
    foo: Boolean,
    bar: {
      type: Boolean,
    },
    baz: {
      type: Number,
      default: 1,
    },
    qux: {
      type: [Boolean, String],
      default: true,
    },
  },
};
</script>
