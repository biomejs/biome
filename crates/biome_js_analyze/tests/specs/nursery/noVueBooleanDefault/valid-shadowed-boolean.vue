<script>
/* should not generate diagnostics */
const Boolean = String;

export default {
  props: {
    foo: {
      type: Boolean,
      default: "foo",
    },
  },
};
</script>
