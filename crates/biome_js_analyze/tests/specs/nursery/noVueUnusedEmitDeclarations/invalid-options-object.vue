<!-- should generate diagnostics -->
<script>
export default {
  emits: {
    foo: null,
    bar: (value) => typeof value === "string",
    baz() {
      return true;
    },
  },
};
</script>
