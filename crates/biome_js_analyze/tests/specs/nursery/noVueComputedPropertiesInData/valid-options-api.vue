<script>
/* should not generate diagnostics */
export default {
  props: ["prop"],
  data() {
    const missing = this.missing;
    const fromProp = this.prop;
    const fromMethod = this.method();
    return {
      later: () => this.foo,
      laterFunction() {
        return this.foo;
      },
      get laterGetter() {
        return this.foo;
      },
    };
  },
  computed: {
    foo() {
      return this.prop;
    },
  },
  methods: {
    method() {
      return this.foo;
    },
  },
};
</script>
