<script>
/* should not generate diagnostics */
export default {
  ...foo,
  emits: {
    ...test(),
    foo: (payload) => typeof payload === "object",
  },
};
</script>
