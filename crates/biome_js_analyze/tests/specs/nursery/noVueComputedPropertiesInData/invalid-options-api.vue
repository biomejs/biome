<script>
/* should generate diagnostics */
export default {
  data() {
    const foo = this.foo;
    const nested = this.foo.bar;
    const quoted = this["foo"];
    if (this.foo) {
      console.log("read");
    }
    return {
      copy: this.foo,
      called: this.bar(),
    };
    this.foo = 1;
  },
  computed: {
    foo() {
      return 1;
    },
    bar: {
      get() {
        return () => 2;
      },
      set(value) {},
    },
  },
};
</script>
