<script>
/* should not generate diagnostics */
export default {
  data: () => ({
    foo: this.foo,
  }),
  computed: {
    foo() {
      return 1;
    },
  },
};
</script>
