<script>
/* should not generate diagnostics */
const Foo = {
  props: {
    a: String,
    b: {
      default() {
        return this.a;
      },
    },
  },
};
</script>
